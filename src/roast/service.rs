use crate::cornucopia::queries::roast::{
    all_roast_levels, all_roast_steps, all_roasts, delete_roast, delete_roast_step,
    find_roast_by_id, find_roast_step_by_id, insert_roast, insert_roast_step, update_roast,
    update_roast_step, InsertRoastParams, InsertRoastStepParams, UpdateRoastParams,
    UpdateRoastStepParams,
};
use crate::roast::model::{Roast, RoastLevel, RoastStep};
use anyhow::anyhow;
use cornucopia_async::Params;
use deadpool_postgres::Pool;
use std::error::Error;
use uuid::Uuid;

impl From<crate::cornucopia::queries::roast::Roast> for Roast {
    fn from(r: crate::cornucopia::queries::roast::Roast) -> Self {
        Roast {
            roast_id: r.roast_id,
            bean_id: r.bean_id,
            roast_level_id: r.roast_level_id,
            ts: r.ts,
        }
    }
}

impl From<crate::cornucopia::queries::roast::RoastStep> for RoastStep {
    fn from(r: crate::cornucopia::queries::roast::RoastStep) -> Self {
        RoastStep {
            roast_step_id: r.roast_step_id,
            roast_id: r.roast_id,
            position: r.position,
            description: r.description,
            time: r.time,
            fan_speed: r.fan_speed,
            temp_setting: r.temp_setting,
            temperature: r.temperature,
        }
    }
}

impl From<crate::cornucopia::queries::roast::RoastLevel> for RoastLevel {
    fn from(r: crate::cornucopia::queries::roast::RoastLevel) -> Self {
        RoastLevel {
            roast_level_id: r.roast_level_id,
            name: r.name,
            description: r.description,
        }
    }
}

#[derive(Clone)]
pub struct RoastService {
    pub(crate) db_pool: Pool,
}

impl RoastService {
    pub async fn get_all_roasts(&self) -> Result<Vec<Roast>, Box<dyn Error>> {
        let client = self.db_pool.get().await?;

        Ok(all_roasts()
            .bind(&client)
            .all()
            .await?
            .into_iter()
            .map(Roast::from)
            .collect())
    }

    pub(crate) async fn get_roast_by_id(&self, id: Uuid) -> Result<Option<Roast>, Box<dyn Error>> {
        let client = self.db_pool.get().await?;

        Ok(find_roast_by_id()
            .bind(&client, &id)
            .opt()
            .await?
            .map(Roast::from))
    }

    pub(crate) async fn create_roast(&self, roast: Roast) -> Result<Roast, Box<dyn Error>> {
        let client = self.db_pool.get().await?;

        Ok(insert_roast()
            .params(
                &client,
                &InsertRoastParams {
                    roast_id: roast.roast_id,
                    bean_id: roast.bean_id,
                    roast_level_id: roast.roast_level_id,
                    ts: roast.ts,
                },
            )
            .one()
            .await
            .map(Roast::from)?)
    }

    pub(crate) async fn update_roast(
        &self,
        id: Uuid,
        roast: Roast,
    ) -> Result<Roast, Box<dyn Error>> {
        let client = self.db_pool.get().await?;

        Ok(update_roast()
            .params(
                &client,
                &UpdateRoastParams {
                    roast_id: id,
                    bean_id: roast.bean_id,
                    roast_level_id: roast.roast_level_id,
                    ts: roast.ts,
                },
            )
            .one()
            .await
            .map(Roast::from)?)
    }

    pub(crate) async fn delete_roast(&self, id: Uuid) -> Result<(), Box<dyn Error>> {
        let client = self.db_pool.get().await?;

        match delete_roast().bind(&client, &id).await {
            Ok(_) => Ok(()),
            Err(e) => Err(Box::from(anyhow!("Failed to delete roast: {}", e))),
        }
    }

    pub async fn get_all_roast_levels(&self) -> Result<Vec<RoastLevel>, Box<dyn Error>> {
        let client = self.db_pool.get().await?;

        Ok(all_roast_levels()
            .bind(&client)
            .all()
            .await?
            .into_iter()
            .map(RoastLevel::from)
            .collect())
    }

    pub async fn get_all_roast_steps(
        &self,
        roast_id: Uuid,
    ) -> Result<Vec<RoastStep>, Box<dyn Error>> {
        let client = self.db_pool.get().await?;

        Ok(all_roast_steps()
            .bind(&client, &roast_id)
            .all()
            .await?
            .into_iter()
            .map(RoastStep::from)
            .collect())
    }

    pub async fn get_roast_step(
        &self,
        roast_step_id: Uuid,
    ) -> Result<Vec<RoastStep>, Box<dyn Error>> {
        let client = self.db_pool.get().await?;

        Ok(find_roast_step_by_id()
            .bind(&client, &roast_step_id)
            .all()
            .await?
            .into_iter()
            .map(RoastStep::from)
            .collect())
    }

    pub(crate) async fn create_roast_step(
        &self,
        step: RoastStep,
    ) -> Result<RoastStep, Box<dyn Error>> {
        let client = self.db_pool.get().await?;

        Ok(insert_roast_step()
            .params(
                &client,
                &InsertRoastStepParams {
                    roast_step_id: step.roast_step_id,
                    roast_id: step.roast_id,
                    position: step.position,
                    description: step.description,
                    time: step.time,
                    fan_speed: step.fan_speed,
                    temp_setting: step.temp_setting,
                    temperature: step.temperature,
                },
            )
            .one()
            .await
            .map(RoastStep::from)?)
    }

    pub(crate) async fn update_roast_step(
        &self,
        step: RoastStep,
    ) -> Result<RoastStep, Box<dyn Error>> {
        let client = self.db_pool.get().await?;

        Ok(update_roast_step()
            .params(
                &client,
                &UpdateRoastStepParams {
                    roast_step_id: step.roast_step_id,
                    roast_id: step.roast_id,
                    position: step.position,
                    description: step.description,
                    time: step.time,
                    fan_speed: step.fan_speed,
                    temp_setting: step.temp_setting,
                    temperature: step.temperature,
                },
            )
            .one()
            .await
            .map(RoastStep::from)?)
    }

    pub(crate) async fn delete_roast_step(&self, id: Uuid) -> Result<(), Box<dyn Error>> {
        let client = self.db_pool.get().await?;

        match delete_roast_step().bind(&client, &id).await {
            Ok(_) => Ok(()),
            Err(e) => Err(Box::from(anyhow!("Failed to delete roast step: {}", e))),
        }
    }
}
