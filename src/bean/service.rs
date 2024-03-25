use crate::bean::model::Bean;
use crate::cornucopia::queries::bean::{Bean as BeanRow, InsertBeanParams};
use cornucopia_async::Params;
use deadpool_postgres::Pool;
use std::error::Error;
use uuid::Uuid;

impl From<BeanRow> for Bean {
    fn from(b: BeanRow) -> Self {
        Bean {
            bean_id: b.bean_id,
            name: b.name,
            description: b.description,
            ts: b.ts,
            region: b.region,
            grade: b.grade,
        }
    }
}

#[derive(Clone)]
pub struct BeanService {
    pub(crate) db_pool: Pool,
}

impl BeanService {
    pub async fn get_beans(&self) -> Result<Vec<Bean>, Box<dyn Error>> {
        let client = self.db_pool.get().await?;

        Ok(crate::cornucopia::queries::bean::all_beans()
            .bind(&client)
            .all()
            .await?
            .into_iter()
            .map(Bean::from)
            .collect())
    }

    pub(crate) async fn create(&self, bean: Bean) -> Result<Bean, Box<dyn Error>> {
        let client = self.db_pool.get().await?;

        Ok(crate::cornucopia::queries::bean::insert_bean()
            .params(
                &client,
                &InsertBeanParams {
                    bean_id: bean.bean_id,
                    name: bean.name,
                    description: bean.description.as_deref(),
                    ts: bean.ts,
                    region: bean.region.as_deref(),
                    grade: bean.grade.as_deref(),
                },
            )
            .one()
            .await
            .map(Bean::from)?)
    }

    pub async fn get_bean_by_id(&self, id: Uuid) -> Result<Bean, Box<dyn Error>> {
        let client = self.db_pool.get().await?;

        Ok(crate::cornucopia::queries::bean::find_bean_by_id()
            .bind(&client, &id)
            .one()
            .await
            .map(Bean::from)?)
    }
}
