// This file was generated with `cornucopia`. Do not modify.

#[allow(clippy::all, clippy::pedantic)]
#[allow(unused_variables)]
#[allow(unused_imports)]
#[allow(dead_code)]
pub mod types {}
#[allow(clippy::all, clippy::pedantic)]
#[allow(unused_variables)]
#[allow(unused_imports)]
#[allow(dead_code)]
pub mod queries {
    pub mod bean {
        use cornucopia_async::GenericClient;
        use futures;
        use futures::{StreamExt, TryStreamExt};
        #[derive(Debug)]
        pub struct InsertBeanParams<
            T1: cornucopia_async::StringSql,
            T2: cornucopia_async::StringSql,
            T3: cornucopia_async::StringSql,
            T4: cornucopia_async::StringSql,
        > {
            pub bean_id: uuid::Uuid,
            pub name: T1,
            pub description: Option<T2>,
            pub ts: time::OffsetDateTime,
            pub region: Option<T3>,
            pub grade: Option<T4>,
        }
        #[derive(serde::Serialize, Debug, Clone, PartialEq)]
        pub struct Bean {
            pub bean_id: uuid::Uuid,
            pub name: String,
            pub description: Option<String>,
            pub ts: time::OffsetDateTime,
            pub region: Option<String>,
            pub grade: Option<String>,
        }
        pub struct BeanBorrowed<'a> {
            pub bean_id: uuid::Uuid,
            pub name: &'a str,
            pub description: Option<&'a str>,
            pub ts: time::OffsetDateTime,
            pub region: Option<&'a str>,
            pub grade: Option<&'a str>,
        }
        impl<'a> From<BeanBorrowed<'a>> for Bean {
            fn from(
                BeanBorrowed {
                    bean_id,
                    name,
                    description,
                    ts,
                    region,
                    grade,
                }: BeanBorrowed<'a>,
            ) -> Self {
                Self {
                    bean_id,
                    name: name.into(),
                    description: description.map(|v| v.into()),
                    ts,
                    region: region.map(|v| v.into()),
                    grade: grade.map(|v| v.into()),
                }
            }
        }
        pub struct BeanQuery<'a, C: GenericClient, T, const N: usize> {
            client: &'a C,
            params: [&'a (dyn postgres_types::ToSql + Sync); N],
            stmt: &'a mut cornucopia_async::private::Stmt,
            extractor: fn(&tokio_postgres::Row) -> BeanBorrowed,
            mapper: fn(BeanBorrowed) -> T,
        }
        impl<'a, C, T: 'a, const N: usize> BeanQuery<'a, C, T, N>
        where
            C: GenericClient,
        {
            pub fn map<R>(self, mapper: fn(BeanBorrowed) -> R) -> BeanQuery<'a, C, R, N> {
                BeanQuery {
                    client: self.client,
                    params: self.params,
                    stmt: self.stmt,
                    extractor: self.extractor,
                    mapper,
                }
            }
            pub async fn one(self) -> Result<T, tokio_postgres::Error> {
                let stmt = self.stmt.prepare(self.client).await?;
                let row = self.client.query_one(stmt, &self.params).await?;
                Ok((self.mapper)((self.extractor)(&row)))
            }
            pub async fn all(self) -> Result<Vec<T>, tokio_postgres::Error> {
                self.iter().await?.try_collect().await
            }
            pub async fn opt(self) -> Result<Option<T>, tokio_postgres::Error> {
                let stmt = self.stmt.prepare(self.client).await?;
                Ok(self
                    .client
                    .query_opt(stmt, &self.params)
                    .await?
                    .map(|row| (self.mapper)((self.extractor)(&row))))
            }
            pub async fn iter(
                self,
            ) -> Result<
                impl futures::Stream<Item = Result<T, tokio_postgres::Error>> + 'a,
                tokio_postgres::Error,
            > {
                let stmt = self.stmt.prepare(self.client).await?;
                let it = self
                    .client
                    .query_raw(stmt, cornucopia_async::private::slice_iter(&self.params))
                    .await?
                    .map(move |res| res.map(|row| (self.mapper)((self.extractor)(&row))))
                    .into_stream();
                Ok(it)
            }
        }
        pub fn all_beans() -> AllBeansStmt {
            AllBeansStmt(cornucopia_async::private::Stmt::new(
                "select bean_id, name, description, ts, region, grade
from core.bean",
            ))
        }
        pub struct AllBeansStmt(cornucopia_async::private::Stmt);
        impl AllBeansStmt {
            pub fn bind<'a, C: GenericClient>(
                &'a mut self,
                client: &'a C,
            ) -> BeanQuery<'a, C, Bean, 0> {
                BeanQuery {
                    client,
                    params: [],
                    stmt: &mut self.0,
                    extractor: |row| BeanBorrowed {
                        bean_id: row.get(0),
                        name: row.get(1),
                        description: row.get(2),
                        ts: row.get(3),
                        region: row.get(4),
                        grade: row.get(5),
                    },
                    mapper: |it| <Bean>::from(it),
                }
            }
        }
        pub fn find_bean_by_id() -> FindBeanByIdStmt {
            FindBeanByIdStmt(cornucopia_async::private::Stmt::new(
                "select bean_id, name, description, ts, region, grade
from core.bean
where bean_id = $1",
            ))
        }
        pub struct FindBeanByIdStmt(cornucopia_async::private::Stmt);
        impl FindBeanByIdStmt {
            pub fn bind<'a, C: GenericClient>(
                &'a mut self,
                client: &'a C,
                bean_id: &'a uuid::Uuid,
            ) -> BeanQuery<'a, C, Bean, 1> {
                BeanQuery {
                    client,
                    params: [bean_id],
                    stmt: &mut self.0,
                    extractor: |row| BeanBorrowed {
                        bean_id: row.get(0),
                        name: row.get(1),
                        description: row.get(2),
                        ts: row.get(3),
                        region: row.get(4),
                        grade: row.get(5),
                    },
                    mapper: |it| <Bean>::from(it),
                }
            }
        }
        pub fn insert_bean() -> InsertBeanStmt {
            InsertBeanStmt(cornucopia_async::private::Stmt::new(
                "insert into core.bean (bean_id, name, description, ts, region, grade)
values ($1, $2, $3, $4, $5, $6)
returning
    bean_id,
    name,
    description,
    ts,
    region,
    grade",
            ))
        }
        pub struct InsertBeanStmt(cornucopia_async::private::Stmt);
        impl InsertBeanStmt {
            pub fn bind<
                'a,
                C: GenericClient,
                T1: cornucopia_async::StringSql,
                T2: cornucopia_async::StringSql,
                T3: cornucopia_async::StringSql,
                T4: cornucopia_async::StringSql,
            >(
                &'a mut self,
                client: &'a C,
                bean_id: &'a uuid::Uuid,
                name: &'a T1,
                description: &'a Option<T2>,
                ts: &'a time::OffsetDateTime,
                region: &'a Option<T3>,
                grade: &'a Option<T4>,
            ) -> BeanQuery<'a, C, Bean, 6> {
                BeanQuery {
                    client,
                    params: [bean_id, name, description, ts, region, grade],
                    stmt: &mut self.0,
                    extractor: |row| BeanBorrowed {
                        bean_id: row.get(0),
                        name: row.get(1),
                        description: row.get(2),
                        ts: row.get(3),
                        region: row.get(4),
                        grade: row.get(5),
                    },
                    mapper: |it| <Bean>::from(it),
                }
            }
        }
        impl<
                'a,
                C: GenericClient,
                T1: cornucopia_async::StringSql,
                T2: cornucopia_async::StringSql,
                T3: cornucopia_async::StringSql,
                T4: cornucopia_async::StringSql,
            >
            cornucopia_async::Params<
                'a,
                InsertBeanParams<T1, T2, T3, T4>,
                BeanQuery<'a, C, Bean, 6>,
                C,
            > for InsertBeanStmt
        {
            fn params(
                &'a mut self,
                client: &'a C,
                params: &'a InsertBeanParams<T1, T2, T3, T4>,
            ) -> BeanQuery<'a, C, Bean, 6> {
                self.bind(
                    client,
                    &params.bean_id,
                    &params.name,
                    &params.description,
                    &params.ts,
                    &params.region,
                    &params.grade,
                )
            }
        }
    }
    pub mod roast {
        use cornucopia_async::GenericClient;
        use futures;
        use futures::{StreamExt, TryStreamExt};
        #[derive(Clone, Copy, Debug)]
        pub struct InsertRoastParams {
            pub roast_id: uuid::Uuid,
            pub bean_id: uuid::Uuid,
            pub roast_level_id: uuid::Uuid,
            pub ts: time::OffsetDateTime,
        }
        #[derive(Debug)]
        pub struct InsertRoastStepParams<T1: cornucopia_async::StringSql> {
            pub roast_step_id: uuid::Uuid,
            pub roast_id: uuid::Uuid,
            pub position: i32,
            pub description: T1,
            pub time: i64,
            pub fan_speed: i32,
            pub temp_setting: i32,
            pub temperature: rust_decimal::Decimal,
        }
        #[derive(serde::Serialize, Debug, Clone, PartialEq)]
        pub struct AllRoasts {
            pub roast_id: uuid::Uuid,
            pub bean_id: uuid::Uuid,
            pub roast_ts: time::OffsetDateTime,
            pub roast_level_id: uuid::Uuid,
            pub roast_level_name: String,
            pub roast_level_description: String,
        }
        pub struct AllRoastsBorrowed<'a> {
            pub roast_id: uuid::Uuid,
            pub bean_id: uuid::Uuid,
            pub roast_ts: time::OffsetDateTime,
            pub roast_level_id: uuid::Uuid,
            pub roast_level_name: &'a str,
            pub roast_level_description: &'a str,
        }
        impl<'a> From<AllRoastsBorrowed<'a>> for AllRoasts {
            fn from(
                AllRoastsBorrowed {
                    roast_id,
                    bean_id,
                    roast_ts,
                    roast_level_id,
                    roast_level_name,
                    roast_level_description,
                }: AllRoastsBorrowed<'a>,
            ) -> Self {
                Self {
                    roast_id,
                    bean_id,
                    roast_ts,
                    roast_level_id,
                    roast_level_name: roast_level_name.into(),
                    roast_level_description: roast_level_description.into(),
                }
            }
        }
        pub struct AllRoastsQuery<'a, C: GenericClient, T, const N: usize> {
            client: &'a C,
            params: [&'a (dyn postgres_types::ToSql + Sync); N],
            stmt: &'a mut cornucopia_async::private::Stmt,
            extractor: fn(&tokio_postgres::Row) -> AllRoastsBorrowed,
            mapper: fn(AllRoastsBorrowed) -> T,
        }
        impl<'a, C, T: 'a, const N: usize> AllRoastsQuery<'a, C, T, N>
        where
            C: GenericClient,
        {
            pub fn map<R>(self, mapper: fn(AllRoastsBorrowed) -> R) -> AllRoastsQuery<'a, C, R, N> {
                AllRoastsQuery {
                    client: self.client,
                    params: self.params,
                    stmt: self.stmt,
                    extractor: self.extractor,
                    mapper,
                }
            }
            pub async fn one(self) -> Result<T, tokio_postgres::Error> {
                let stmt = self.stmt.prepare(self.client).await?;
                let row = self.client.query_one(stmt, &self.params).await?;
                Ok((self.mapper)((self.extractor)(&row)))
            }
            pub async fn all(self) -> Result<Vec<T>, tokio_postgres::Error> {
                self.iter().await?.try_collect().await
            }
            pub async fn opt(self) -> Result<Option<T>, tokio_postgres::Error> {
                let stmt = self.stmt.prepare(self.client).await?;
                Ok(self
                    .client
                    .query_opt(stmt, &self.params)
                    .await?
                    .map(|row| (self.mapper)((self.extractor)(&row))))
            }
            pub async fn iter(
                self,
            ) -> Result<
                impl futures::Stream<Item = Result<T, tokio_postgres::Error>> + 'a,
                tokio_postgres::Error,
            > {
                let stmt = self.stmt.prepare(self.client).await?;
                let it = self
                    .client
                    .query_raw(stmt, cornucopia_async::private::slice_iter(&self.params))
                    .await?
                    .map(move |res| res.map(|row| (self.mapper)((self.extractor)(&row))))
                    .into_stream();
                Ok(it)
            }
        }
        #[derive(serde::Serialize, Debug, Clone, PartialEq)]
        pub struct FindRoastById {
            pub roast_id: uuid::Uuid,
            pub bean_id: uuid::Uuid,
            pub roast_ts: time::OffsetDateTime,
            pub roast_level_id: uuid::Uuid,
            pub roast_level_name: String,
            pub roast_level_description: String,
        }
        pub struct FindRoastByIdBorrowed<'a> {
            pub roast_id: uuid::Uuid,
            pub bean_id: uuid::Uuid,
            pub roast_ts: time::OffsetDateTime,
            pub roast_level_id: uuid::Uuid,
            pub roast_level_name: &'a str,
            pub roast_level_description: &'a str,
        }
        impl<'a> From<FindRoastByIdBorrowed<'a>> for FindRoastById {
            fn from(
                FindRoastByIdBorrowed {
                    roast_id,
                    bean_id,
                    roast_ts,
                    roast_level_id,
                    roast_level_name,
                    roast_level_description,
                }: FindRoastByIdBorrowed<'a>,
            ) -> Self {
                Self {
                    roast_id,
                    bean_id,
                    roast_ts,
                    roast_level_id,
                    roast_level_name: roast_level_name.into(),
                    roast_level_description: roast_level_description.into(),
                }
            }
        }
        pub struct FindRoastByIdQuery<'a, C: GenericClient, T, const N: usize> {
            client: &'a C,
            params: [&'a (dyn postgres_types::ToSql + Sync); N],
            stmt: &'a mut cornucopia_async::private::Stmt,
            extractor: fn(&tokio_postgres::Row) -> FindRoastByIdBorrowed,
            mapper: fn(FindRoastByIdBorrowed) -> T,
        }
        impl<'a, C, T: 'a, const N: usize> FindRoastByIdQuery<'a, C, T, N>
        where
            C: GenericClient,
        {
            pub fn map<R>(
                self,
                mapper: fn(FindRoastByIdBorrowed) -> R,
            ) -> FindRoastByIdQuery<'a, C, R, N> {
                FindRoastByIdQuery {
                    client: self.client,
                    params: self.params,
                    stmt: self.stmt,
                    extractor: self.extractor,
                    mapper,
                }
            }
            pub async fn one(self) -> Result<T, tokio_postgres::Error> {
                let stmt = self.stmt.prepare(self.client).await?;
                let row = self.client.query_one(stmt, &self.params).await?;
                Ok((self.mapper)((self.extractor)(&row)))
            }
            pub async fn all(self) -> Result<Vec<T>, tokio_postgres::Error> {
                self.iter().await?.try_collect().await
            }
            pub async fn opt(self) -> Result<Option<T>, tokio_postgres::Error> {
                let stmt = self.stmt.prepare(self.client).await?;
                Ok(self
                    .client
                    .query_opt(stmt, &self.params)
                    .await?
                    .map(|row| (self.mapper)((self.extractor)(&row))))
            }
            pub async fn iter(
                self,
            ) -> Result<
                impl futures::Stream<Item = Result<T, tokio_postgres::Error>> + 'a,
                tokio_postgres::Error,
            > {
                let stmt = self.stmt.prepare(self.client).await?;
                let it = self
                    .client
                    .query_raw(stmt, cornucopia_async::private::slice_iter(&self.params))
                    .await?
                    .map(move |res| res.map(|row| (self.mapper)((self.extractor)(&row))))
                    .into_stream();
                Ok(it)
            }
        }
        #[derive(serde::Serialize, Debug, Clone, PartialEq)]
        pub struct AllRoastSteps {
            pub roast_step_id: uuid::Uuid,
            pub roast_id: uuid::Uuid,
            pub position: i32,
            pub description: String,
            pub time: i64,
            pub fan_speed: i32,
            pub temp_setting: i32,
            pub temperature: rust_decimal::Decimal,
        }
        pub struct AllRoastStepsBorrowed<'a> {
            pub roast_step_id: uuid::Uuid,
            pub roast_id: uuid::Uuid,
            pub position: i32,
            pub description: &'a str,
            pub time: i64,
            pub fan_speed: i32,
            pub temp_setting: i32,
            pub temperature: rust_decimal::Decimal,
        }
        impl<'a> From<AllRoastStepsBorrowed<'a>> for AllRoastSteps {
            fn from(
                AllRoastStepsBorrowed {
                    roast_step_id,
                    roast_id,
                    position,
                    description,
                    time,
                    fan_speed,
                    temp_setting,
                    temperature,
                }: AllRoastStepsBorrowed<'a>,
            ) -> Self {
                Self {
                    roast_step_id,
                    roast_id,
                    position,
                    description: description.into(),
                    time,
                    fan_speed,
                    temp_setting,
                    temperature,
                }
            }
        }
        pub struct AllRoastStepsQuery<'a, C: GenericClient, T, const N: usize> {
            client: &'a C,
            params: [&'a (dyn postgres_types::ToSql + Sync); N],
            stmt: &'a mut cornucopia_async::private::Stmt,
            extractor: fn(&tokio_postgres::Row) -> AllRoastStepsBorrowed,
            mapper: fn(AllRoastStepsBorrowed) -> T,
        }
        impl<'a, C, T: 'a, const N: usize> AllRoastStepsQuery<'a, C, T, N>
        where
            C: GenericClient,
        {
            pub fn map<R>(
                self,
                mapper: fn(AllRoastStepsBorrowed) -> R,
            ) -> AllRoastStepsQuery<'a, C, R, N> {
                AllRoastStepsQuery {
                    client: self.client,
                    params: self.params,
                    stmt: self.stmt,
                    extractor: self.extractor,
                    mapper,
                }
            }
            pub async fn one(self) -> Result<T, tokio_postgres::Error> {
                let stmt = self.stmt.prepare(self.client).await?;
                let row = self.client.query_one(stmt, &self.params).await?;
                Ok((self.mapper)((self.extractor)(&row)))
            }
            pub async fn all(self) -> Result<Vec<T>, tokio_postgres::Error> {
                self.iter().await?.try_collect().await
            }
            pub async fn opt(self) -> Result<Option<T>, tokio_postgres::Error> {
                let stmt = self.stmt.prepare(self.client).await?;
                Ok(self
                    .client
                    .query_opt(stmt, &self.params)
                    .await?
                    .map(|row| (self.mapper)((self.extractor)(&row))))
            }
            pub async fn iter(
                self,
            ) -> Result<
                impl futures::Stream<Item = Result<T, tokio_postgres::Error>> + 'a,
                tokio_postgres::Error,
            > {
                let stmt = self.stmt.prepare(self.client).await?;
                let it = self
                    .client
                    .query_raw(stmt, cornucopia_async::private::slice_iter(&self.params))
                    .await?
                    .map(move |res| res.map(|row| (self.mapper)((self.extractor)(&row))))
                    .into_stream();
                Ok(it)
            }
        }
        #[derive(serde::Serialize, Debug, Clone, PartialEq)]
        pub struct InsertRoast {
            pub roast_id: uuid::Uuid,
            pub bean_id: uuid::Uuid,
            pub roast_ts: time::OffsetDateTime,
            pub roast_level_id: uuid::Uuid,
            pub roast_level_name: String,
            pub roast_level_description: String,
        }
        pub struct InsertRoastBorrowed<'a> {
            pub roast_id: uuid::Uuid,
            pub bean_id: uuid::Uuid,
            pub roast_ts: time::OffsetDateTime,
            pub roast_level_id: uuid::Uuid,
            pub roast_level_name: &'a str,
            pub roast_level_description: &'a str,
        }
        impl<'a> From<InsertRoastBorrowed<'a>> for InsertRoast {
            fn from(
                InsertRoastBorrowed {
                    roast_id,
                    bean_id,
                    roast_ts,
                    roast_level_id,
                    roast_level_name,
                    roast_level_description,
                }: InsertRoastBorrowed<'a>,
            ) -> Self {
                Self {
                    roast_id,
                    bean_id,
                    roast_ts,
                    roast_level_id,
                    roast_level_name: roast_level_name.into(),
                    roast_level_description: roast_level_description.into(),
                }
            }
        }
        pub struct InsertRoastQuery<'a, C: GenericClient, T, const N: usize> {
            client: &'a C,
            params: [&'a (dyn postgres_types::ToSql + Sync); N],
            stmt: &'a mut cornucopia_async::private::Stmt,
            extractor: fn(&tokio_postgres::Row) -> InsertRoastBorrowed,
            mapper: fn(InsertRoastBorrowed) -> T,
        }
        impl<'a, C, T: 'a, const N: usize> InsertRoastQuery<'a, C, T, N>
        where
            C: GenericClient,
        {
            pub fn map<R>(
                self,
                mapper: fn(InsertRoastBorrowed) -> R,
            ) -> InsertRoastQuery<'a, C, R, N> {
                InsertRoastQuery {
                    client: self.client,
                    params: self.params,
                    stmt: self.stmt,
                    extractor: self.extractor,
                    mapper,
                }
            }
            pub async fn one(self) -> Result<T, tokio_postgres::Error> {
                let stmt = self.stmt.prepare(self.client).await?;
                let row = self.client.query_one(stmt, &self.params).await?;
                Ok((self.mapper)((self.extractor)(&row)))
            }
            pub async fn all(self) -> Result<Vec<T>, tokio_postgres::Error> {
                self.iter().await?.try_collect().await
            }
            pub async fn opt(self) -> Result<Option<T>, tokio_postgres::Error> {
                let stmt = self.stmt.prepare(self.client).await?;
                Ok(self
                    .client
                    .query_opt(stmt, &self.params)
                    .await?
                    .map(|row| (self.mapper)((self.extractor)(&row))))
            }
            pub async fn iter(
                self,
            ) -> Result<
                impl futures::Stream<Item = Result<T, tokio_postgres::Error>> + 'a,
                tokio_postgres::Error,
            > {
                let stmt = self.stmt.prepare(self.client).await?;
                let it = self
                    .client
                    .query_raw(stmt, cornucopia_async::private::slice_iter(&self.params))
                    .await?
                    .map(move |res| res.map(|row| (self.mapper)((self.extractor)(&row))))
                    .into_stream();
                Ok(it)
            }
        }
        #[derive(serde::Serialize, Debug, Clone, PartialEq)]
        pub struct InsertRoastStep {
            pub roast_step_id: uuid::Uuid,
            pub roast_id: uuid::Uuid,
            pub position: i32,
            pub description: String,
            pub time: i64,
            pub fan_speed: i32,
            pub temp_setting: i32,
            pub temperature: rust_decimal::Decimal,
        }
        pub struct InsertRoastStepBorrowed<'a> {
            pub roast_step_id: uuid::Uuid,
            pub roast_id: uuid::Uuid,
            pub position: i32,
            pub description: &'a str,
            pub time: i64,
            pub fan_speed: i32,
            pub temp_setting: i32,
            pub temperature: rust_decimal::Decimal,
        }
        impl<'a> From<InsertRoastStepBorrowed<'a>> for InsertRoastStep {
            fn from(
                InsertRoastStepBorrowed {
                    roast_step_id,
                    roast_id,
                    position,
                    description,
                    time,
                    fan_speed,
                    temp_setting,
                    temperature,
                }: InsertRoastStepBorrowed<'a>,
            ) -> Self {
                Self {
                    roast_step_id,
                    roast_id,
                    position,
                    description: description.into(),
                    time,
                    fan_speed,
                    temp_setting,
                    temperature,
                }
            }
        }
        pub struct InsertRoastStepQuery<'a, C: GenericClient, T, const N: usize> {
            client: &'a C,
            params: [&'a (dyn postgres_types::ToSql + Sync); N],
            stmt: &'a mut cornucopia_async::private::Stmt,
            extractor: fn(&tokio_postgres::Row) -> InsertRoastStepBorrowed,
            mapper: fn(InsertRoastStepBorrowed) -> T,
        }
        impl<'a, C, T: 'a, const N: usize> InsertRoastStepQuery<'a, C, T, N>
        where
            C: GenericClient,
        {
            pub fn map<R>(
                self,
                mapper: fn(InsertRoastStepBorrowed) -> R,
            ) -> InsertRoastStepQuery<'a, C, R, N> {
                InsertRoastStepQuery {
                    client: self.client,
                    params: self.params,
                    stmt: self.stmt,
                    extractor: self.extractor,
                    mapper,
                }
            }
            pub async fn one(self) -> Result<T, tokio_postgres::Error> {
                let stmt = self.stmt.prepare(self.client).await?;
                let row = self.client.query_one(stmt, &self.params).await?;
                Ok((self.mapper)((self.extractor)(&row)))
            }
            pub async fn all(self) -> Result<Vec<T>, tokio_postgres::Error> {
                self.iter().await?.try_collect().await
            }
            pub async fn opt(self) -> Result<Option<T>, tokio_postgres::Error> {
                let stmt = self.stmt.prepare(self.client).await?;
                Ok(self
                    .client
                    .query_opt(stmt, &self.params)
                    .await?
                    .map(|row| (self.mapper)((self.extractor)(&row))))
            }
            pub async fn iter(
                self,
            ) -> Result<
                impl futures::Stream<Item = Result<T, tokio_postgres::Error>> + 'a,
                tokio_postgres::Error,
            > {
                let stmt = self.stmt.prepare(self.client).await?;
                let it = self
                    .client
                    .query_raw(stmt, cornucopia_async::private::slice_iter(&self.params))
                    .await?
                    .map(move |res| res.map(|row| (self.mapper)((self.extractor)(&row))))
                    .into_stream();
                Ok(it)
            }
        }
        pub fn all_roasts() -> AllRoastsStmt {
            AllRoastsStmt(cornucopia_async::private::Stmt::new(
                "select r.roast_id        as roast_id,
       r.bean_id         as bean_id,
       r.ts              as roast_ts,
       rl.roast_level_id as roast_level_id,
       rl.name           as roast_level_name,
       rl.description    as roast_level_description
from core.roast r
         inner join core.roast_level rl using (roast_level_id)",
            ))
        }
        pub struct AllRoastsStmt(cornucopia_async::private::Stmt);
        impl AllRoastsStmt {
            pub fn bind<'a, C: GenericClient>(
                &'a mut self,
                client: &'a C,
            ) -> AllRoastsQuery<'a, C, AllRoasts, 0> {
                AllRoastsQuery {
                    client,
                    params: [],
                    stmt: &mut self.0,
                    extractor: |row| AllRoastsBorrowed {
                        roast_id: row.get(0),
                        bean_id: row.get(1),
                        roast_ts: row.get(2),
                        roast_level_id: row.get(3),
                        roast_level_name: row.get(4),
                        roast_level_description: row.get(5),
                    },
                    mapper: |it| <AllRoasts>::from(it),
                }
            }
        }
        pub fn find_roast_by_id() -> FindRoastByIdStmt {
            FindRoastByIdStmt(cornucopia_async::private::Stmt::new(
                "select r.roast_id        as roast_id,
       r.bean_id         as bean_id,
       r.ts              as roast_ts,
       rl.roast_level_id as roast_level_id,
       rl.name           as roast_level_name,
       rl.description    as roast_level_description
from core.roast r
         inner join core.roast_level rl using (roast_level_id)
where roast_id = $1",
            ))
        }
        pub struct FindRoastByIdStmt(cornucopia_async::private::Stmt);
        impl FindRoastByIdStmt {
            pub fn bind<'a, C: GenericClient>(
                &'a mut self,
                client: &'a C,
                roast_id: &'a uuid::Uuid,
            ) -> FindRoastByIdQuery<'a, C, FindRoastById, 1> {
                FindRoastByIdQuery {
                    client,
                    params: [roast_id],
                    stmt: &mut self.0,
                    extractor: |row| FindRoastByIdBorrowed {
                        roast_id: row.get(0),
                        bean_id: row.get(1),
                        roast_ts: row.get(2),
                        roast_level_id: row.get(3),
                        roast_level_name: row.get(4),
                        roast_level_description: row.get(5),
                    },
                    mapper: |it| <FindRoastById>::from(it),
                }
            }
        }
        pub fn all_roast_steps() -> AllRoastStepsStmt {
            AllRoastStepsStmt(cornucopia_async::private::Stmt::new(
                "select roast_step_id,
       roast_id,
       position,
       description,
       time,
       fan_speed,
       temp_setting,
       temperature
from core.roast_step
where roast_id = $1",
            ))
        }
        pub struct AllRoastStepsStmt(cornucopia_async::private::Stmt);
        impl AllRoastStepsStmt {
            pub fn bind<'a, C: GenericClient>(
                &'a mut self,
                client: &'a C,
                roast_id: &'a uuid::Uuid,
            ) -> AllRoastStepsQuery<'a, C, AllRoastSteps, 1> {
                AllRoastStepsQuery {
                    client,
                    params: [roast_id],
                    stmt: &mut self.0,
                    extractor: |row| AllRoastStepsBorrowed {
                        roast_step_id: row.get(0),
                        roast_id: row.get(1),
                        position: row.get(2),
                        description: row.get(3),
                        time: row.get(4),
                        fan_speed: row.get(5),
                        temp_setting: row.get(6),
                        temperature: row.get(7),
                    },
                    mapper: |it| <AllRoastSteps>::from(it),
                }
            }
        }
        pub fn insert_roast() -> InsertRoastStmt {
            InsertRoastStmt(cornucopia_async::private::Stmt::new(
                "with r as (
    insert into core.roast (roast_id, bean_id, roast_level_id, ts)
        values ($1, $2, $3, $4)
        returning roast_id, bean_id, roast_level_id, ts)
select r.roast_id        as roast_id,
       r.bean_id         as bean_id,
       r.ts              as roast_ts,
       rl.roast_level_id as roast_level_id,
       rl.name           as roast_level_name,
       rl.description    as roast_level_description
from r
         inner join core.roast_level rl using (roast_level_id)",
            ))
        }
        pub struct InsertRoastStmt(cornucopia_async::private::Stmt);
        impl InsertRoastStmt {
            pub fn bind<'a, C: GenericClient>(
                &'a mut self,
                client: &'a C,
                roast_id: &'a uuid::Uuid,
                bean_id: &'a uuid::Uuid,
                roast_level_id: &'a uuid::Uuid,
                ts: &'a time::OffsetDateTime,
            ) -> InsertRoastQuery<'a, C, InsertRoast, 4> {
                InsertRoastQuery {
                    client,
                    params: [roast_id, bean_id, roast_level_id, ts],
                    stmt: &mut self.0,
                    extractor: |row| InsertRoastBorrowed {
                        roast_id: row.get(0),
                        bean_id: row.get(1),
                        roast_ts: row.get(2),
                        roast_level_id: row.get(3),
                        roast_level_name: row.get(4),
                        roast_level_description: row.get(5),
                    },
                    mapper: |it| <InsertRoast>::from(it),
                }
            }
        }
        impl<'a, C: GenericClient>
            cornucopia_async::Params<
                'a,
                InsertRoastParams,
                InsertRoastQuery<'a, C, InsertRoast, 4>,
                C,
            > for InsertRoastStmt
        {
            fn params(
                &'a mut self,
                client: &'a C,
                params: &'a InsertRoastParams,
            ) -> InsertRoastQuery<'a, C, InsertRoast, 4> {
                self.bind(
                    client,
                    &params.roast_id,
                    &params.bean_id,
                    &params.roast_level_id,
                    &params.ts,
                )
            }
        }
        pub fn insert_roast_step() -> InsertRoastStepStmt {
            InsertRoastStepStmt(cornucopia_async::private::Stmt::new("insert into core.roast_step(roast_step_id, roast_id, position, description, time, fan_speed, temp_setting, temperature)
values ($1, $2, $3, $4, $5, $6, $7, $8)
returning
    roast_step_id,
    roast_id,
    position,
    description,
    time,
    fan_speed,
    temp_setting,
    temperature"))
        }
        pub struct InsertRoastStepStmt(cornucopia_async::private::Stmt);
        impl InsertRoastStepStmt {
            pub fn bind<'a, C: GenericClient, T1: cornucopia_async::StringSql>(
                &'a mut self,
                client: &'a C,
                roast_step_id: &'a uuid::Uuid,
                roast_id: &'a uuid::Uuid,
                position: &'a i32,
                description: &'a T1,
                time: &'a i64,
                fan_speed: &'a i32,
                temp_setting: &'a i32,
                temperature: &'a rust_decimal::Decimal,
            ) -> InsertRoastStepQuery<'a, C, InsertRoastStep, 8> {
                InsertRoastStepQuery {
                    client,
                    params: [
                        roast_step_id,
                        roast_id,
                        position,
                        description,
                        time,
                        fan_speed,
                        temp_setting,
                        temperature,
                    ],
                    stmt: &mut self.0,
                    extractor: |row| InsertRoastStepBorrowed {
                        roast_step_id: row.get(0),
                        roast_id: row.get(1),
                        position: row.get(2),
                        description: row.get(3),
                        time: row.get(4),
                        fan_speed: row.get(5),
                        temp_setting: row.get(6),
                        temperature: row.get(7),
                    },
                    mapper: |it| <InsertRoastStep>::from(it),
                }
            }
        }
        impl<'a, C: GenericClient, T1: cornucopia_async::StringSql>
            cornucopia_async::Params<
                'a,
                InsertRoastStepParams<T1>,
                InsertRoastStepQuery<'a, C, InsertRoastStep, 8>,
                C,
            > for InsertRoastStepStmt
        {
            fn params(
                &'a mut self,
                client: &'a C,
                params: &'a InsertRoastStepParams<T1>,
            ) -> InsertRoastStepQuery<'a, C, InsertRoastStep, 8> {
                self.bind(
                    client,
                    &params.roast_step_id,
                    &params.roast_id,
                    &params.position,
                    &params.description,
                    &params.time,
                    &params.fan_speed,
                    &params.temp_setting,
                    &params.temperature,
                )
            }
        }
    }
}
