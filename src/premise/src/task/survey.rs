use std::collections::HashMap;

use async_trait::async_trait;
use chrono::{Duration, Utc};
use rand::{prelude::IteratorRandom, Rng};
use serde_json::Number;

use crate::{
	api::{
		form_localization::group::{self, input::InputGroup, output::Value, Type},
		location::upload::UploadLocation,
		task,
	},
	client::Client,
	result,
	task::generator::Generator,
};

pub struct Survey(pub serde_json::Number);

#[async_trait]
impl<'a> super::Task<&'a Vec<&'a InputGroup>> for Survey {
	async fn submit(&self, client: &Client) -> result::Result<()> {
		let reserved_task = client.user.reserve(self.id().clone()).await?.reserved_tasks
			.into_iter().next().unwrap_or({
			let (reservation, task) = client.cache.reservations
				.iter()
				.find_map(|reservation| {
					if &reservation.info.id.0 == self.id() {
						Some((
							reservation,
							client.cache.task_configurations
								.iter()
								.find(|configuration| &configuration.id == self.id())?
						))
					} else {
						None
					}
				}).ok_or(result::error::Error::InvalidTask)?;

			task::reserve::response::ReservedTask {
				reservation: reservation.clone(),
				task: task.clone(),
			}
		});
		let input_groups: Vec<_> = reserved_task.task.form_localization.survey.contexts
			.iter()
			.flat_map(|context| &context.input_groups)
			.collect();
		let mut outputs = Self::generate(&input_groups);
		let output_group_results =
			input_groups
				.iter()
				.flat_map(|input_group| input_group.inputs.iter().map(|input| (&input_group.name, input)))
				.filter_map(|(input_group_name, input)| {
					let value = outputs.remove(&input.name)?;
					let start_time = Utc::now();
					let delay = Duration::milliseconds(rand::thread_rng().gen_range(2000..6000));
					let end_time = start_time + delay;

					std::thread::sleep(delay.to_std().unwrap());

					Some(
						group::output::OutputGroup {
							name: input_group_name.clone(),
							results: vec![
								group::output::Result {
									start_time,
									end_time,
									outputs: vec![
										group::output::Output {
											name: input.name.clone(),
											created_time: end_time,
											location: UploadLocation::new(client.user.location.clone(), client.uptime()),
											value,
											r#type: input.r#type,
											class_name: input.class_name.clone(),
										}
									],
								}
							],
						}
					)
				})
				.collect();
		let reservation_info = task::sync::response::reservation::Info {
			task_info: reserved_task.reservation.info.clone(),
			reservation_id: reserved_task.reservation.id.into(),
		};

		task::submission::submit(
			&client.user.http_client,
			&task::submission::request::Body {
				user_id: client.user.data.id.clone().into(),
				reservation_info: reservation_info.clone(),
				client_submitted_time: Utc::now(),
				output_group_results,
				upload_location: UploadLocation::new(client.user.location.clone(), client.uptime()),
				client_submission_id: uuid::Uuid::new_v4(),
				service_data: Some(reserved_task.task.service_data),
			},
		).await?;
		task::reserve::sync(&client.user.http_client, vec![reservation_info]).await?;

		Ok(())
	}

	fn id(&self) -> &Number {
		&self.0
	}
}

impl<'a> Generator<&'a Vec<&'a InputGroup>> for Survey {
	type Output = HashMap<&'a String, Value>;

	fn generate(input_groups: &'a Vec<&'a InputGroup>) -> Self::Output {
		let mut selected = HashMap::new();

		input_groups
			.iter()
			.flat_map(|input_group| {
				input_group.inputs
					.iter()
					.filter_map(|input| {
						/*
							match &input_group.relevance {
								None | Some(relevance) if relevance.predicate.evaluate(&chosen) => todo()!,
								_ => todo!(),
							}
						*/
						let condition = match &input_group.relevance {
							None => true,
							Some(relevance) if relevance.predicate.evaluate(&selected) => true,
							_ => false
						};

						if condition {
							let options = input.options.as_ref()?;
							let value = match input.r#type {
								Type::SelectOne => either::Left(
									options[rand::thread_rng().gen_range(0..options.len())]
										.value.clone()
								),
								Type::SelectMany => {
									let amount = rand::thread_rng().gen_range(1..options.len() / 2);
									let answers = loop {
										let answers = options.iter().map(|option| option.value.clone())
											.choose_multiple(&mut rand::thread_rng(), amount);

										if !answers.iter().any(|answer| answer == "I do not know" && answer == "Other") {
											break answers;
										}
									};

									either::Right(answers)
								}
								_ => panic!("Invalid type."),
							};

							if let either::Left(value) = &value {
								selected.insert(input.name.clone(), value.clone());
							}

							Some((&input.name, value))
						} else {
							None
						}
					}).collect::<Vec<_>>()
			})
			.collect()
	}
}
