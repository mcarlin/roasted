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
        #[derive(Debug)]
        pub struct UpdateBeanParams<
            T1: cornucopia_async::StringSql,
            T2: cornucopia_async::StringSql,
            T3: cornucopia_async::StringSql,
            T4: cornucopia_async::StringSql,
        > {
            pub name: T1,
            pub description: Option<T2>,
            pub ts: time::OffsetDateTime,
            pub region: Option<T3>,
            pub grade: Option<T4>,
            pub bean_id: uuid::Uuid,
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
        pub fn update_bean() -> UpdateBeanStmt {
            UpdateBeanStmt(cornucopia_async::private::Stmt::new(
                "update core.bean
set (name, description, ts, region, grade) = ($1, $2, $3, $4, $5)
where bean_id = $6
returning
    bean_id,
    name,
    description,
    ts,
    region,
    grade",
            ))
        }
        pub struct UpdateBeanStmt(cornucopia_async::private::Stmt);
        impl UpdateBeanStmt {
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
                name: &'a T1,
                description: &'a Option<T2>,
                ts: &'a time::OffsetDateTime,
                region: &'a Option<T3>,
                grade: &'a Option<T4>,
                bean_id: &'a uuid::Uuid,
            ) -> BeanQuery<'a, C, Bean, 6> {
                BeanQuery {
                    client,
                    params: [name, description, ts, region, grade, bean_id],
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
                UpdateBeanParams<T1, T2, T3, T4>,
                BeanQuery<'a, C, Bean, 6>,
                C,
            > for UpdateBeanStmt
        {
            fn params(
                &'a mut self,
                client: &'a C,
                params: &'a UpdateBeanParams<T1, T2, T3, T4>,
            ) -> BeanQuery<'a, C, Bean, 6> {
                self.bind(
                    client,
                    &params.name,
                    &params.description,
                    &params.ts,
                    &params.region,
                    &params.grade,
                    &params.bean_id,
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
        #[derive(Clone, Copy, Debug)]
        pub struct UpdateRoastParams {
            pub bean_id: uuid::Uuid,
            pub roast_level_id: uuid::Uuid,
            pub ts: time::OffsetDateTime,
            pub roast_id: uuid::Uuid,
        }
        #[derive(Debug)]
        pub struct InsertRoastStepParams<T1: cornucopia_async::StringSql> {
            pub roast_step_id: uuid::Uuid,
            pub roast_id: uuid::Uuid,
            pub position: i32,
            pub description: Option<T1>,
            pub time: i64,
            pub fan_speed: Option<i32>,
            pub temp_setting: Option<i32>,
            pub temperature: Option<rust_decimal::Decimal>,
        }
        #[derive(Debug)]
        pub struct UpdateRoastStepParams<T1: cornucopia_async::StringSql> {
            pub roast_id: uuid::Uuid,
            pub position: i32,
            pub description: Option<T1>,
            pub time: i64,
            pub fan_speed: Option<i32>,
            pub temp_setting: Option<i32>,
            pub temperature: Option<rust_decimal::Decimal>,
            pub roast_step_id: uuid::Uuid,
        }
        #[derive(serde::Serialize, Debug, Clone, PartialEq, Copy)]
        pub struct Roast {
            pub roast_id: uuid::Uuid,
            pub bean_id: uuid::Uuid,
            pub ts: time::OffsetDateTime,
            pub roast_level_id: uuid::Uuid,
        }
        pub struct RoastQuery<'a, C: GenericClient, T, const N: usize> {
            client: &'a C,
            params: [&'a (dyn postgres_types::ToSql + Sync); N],
            stmt: &'a mut cornucopia_async::private::Stmt,
            extractor: fn(&tokio_postgres::Row) -> Roast,
            mapper: fn(Roast) -> T,
        }
        impl<'a, C, T: 'a, const N: usize> RoastQuery<'a, C, T, N>
        where
            C: GenericClient,
        {
            pub fn map<R>(self, mapper: fn(Roast) -> R) -> RoastQuery<'a, C, R, N> {
                RoastQuery {
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
        pub struct RoastLevel {
            pub roast_level_id: uuid::Uuid,
            pub name: String,
            pub description: Option<String>,
        }
        pub struct RoastLevelBorrowed<'a> {
            pub roast_level_id: uuid::Uuid,
            pub name: &'a str,
            pub description: Option<&'a str>,
        }
        impl<'a> From<RoastLevelBorrowed<'a>> for RoastLevel {
            fn from(
                RoastLevelBorrowed {
                    roast_level_id,
                    name,
                    description,
                }: RoastLevelBorrowed<'a>,
            ) -> Self {
                Self {
                    roast_level_id,
                    name: name.into(),
                    description: description.map(|v| v.into()),
                }
            }
        }
        pub struct RoastLevelQuery<'a, C: GenericClient, T, const N: usize> {
            client: &'a C,
            params: [&'a (dyn postgres_types::ToSql + Sync); N],
            stmt: &'a mut cornucopia_async::private::Stmt,
            extractor: fn(&tokio_postgres::Row) -> RoastLevelBorrowed,
            mapper: fn(RoastLevelBorrowed) -> T,
        }
        impl<'a, C, T: 'a, const N: usize> RoastLevelQuery<'a, C, T, N>
        where
            C: GenericClient,
        {
            pub fn map<R>(
                self,
                mapper: fn(RoastLevelBorrowed) -> R,
            ) -> RoastLevelQuery<'a, C, R, N> {
                RoastLevelQuery {
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
        pub struct RoastStep {
            pub roast_step_id: uuid::Uuid,
            pub roast_id: uuid::Uuid,
            pub position: i32,
            pub description: Option<String>,
            pub time: i64,
            pub fan_speed: Option<i32>,
            pub temp_setting: Option<i32>,
            pub temperature: Option<rust_decimal::Decimal>,
        }
        pub struct RoastStepBorrowed<'a> {
            pub roast_step_id: uuid::Uuid,
            pub roast_id: uuid::Uuid,
            pub position: i32,
            pub description: Option<&'a str>,
            pub time: i64,
            pub fan_speed: Option<i32>,
            pub temp_setting: Option<i32>,
            pub temperature: Option<rust_decimal::Decimal>,
        }
        impl<'a> From<RoastStepBorrowed<'a>> for RoastStep {
            fn from(
                RoastStepBorrowed {
                    roast_step_id,
                    roast_id,
                    position,
                    description,
                    time,
                    fan_speed,
                    temp_setting,
                    temperature,
                }: RoastStepBorrowed<'a>,
            ) -> Self {
                Self {
                    roast_step_id,
                    roast_id,
                    position,
                    description: description.map(|v| v.into()),
                    time,
                    fan_speed,
                    temp_setting,
                    temperature,
                }
            }
        }
        pub struct RoastStepQuery<'a, C: GenericClient, T, const N: usize> {
            client: &'a C,
            params: [&'a (dyn postgres_types::ToSql + Sync); N],
            stmt: &'a mut cornucopia_async::private::Stmt,
            extractor: fn(&tokio_postgres::Row) -> RoastStepBorrowed,
            mapper: fn(RoastStepBorrowed) -> T,
        }
        impl<'a, C, T: 'a, const N: usize> RoastStepQuery<'a, C, T, N>
        where
            C: GenericClient,
        {
            pub fn map<R>(self, mapper: fn(RoastStepBorrowed) -> R) -> RoastStepQuery<'a, C, R, N> {
                RoastStepQuery {
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
                "select r.roast_id       as roast_id,
       r.bean_id        as bean_id,
       r.ts             as ts,
       r.roast_level_id as roast_level_id
from core.roast r",
            ))
        }
        pub struct AllRoastsStmt(cornucopia_async::private::Stmt);
        impl AllRoastsStmt {
            pub fn bind<'a, C: GenericClient>(
                &'a mut self,
                client: &'a C,
            ) -> RoastQuery<'a, C, Roast, 0> {
                RoastQuery {
                    client,
                    params: [],
                    stmt: &mut self.0,
                    extractor: |row| Roast {
                        roast_id: row.get(0),
                        bean_id: row.get(1),
                        ts: row.get(2),
                        roast_level_id: row.get(3),
                    },
                    mapper: |it| <Roast>::from(it),
                }
            }
        }
        pub fn find_roast_by_id() -> FindRoastByIdStmt {
            FindRoastByIdStmt(cornucopia_async::private::Stmt::new(
                "select r.roast_id       as roast_id,
       r.bean_id        as bean_id,
       r.ts             as ts,
       r.roast_level_id as roast_level_id
from core.roast r
where roast_id = $1",
            ))
        }
        pub struct FindRoastByIdStmt(cornucopia_async::private::Stmt);
        impl FindRoastByIdStmt {
            pub fn bind<'a, C: GenericClient>(
                &'a mut self,
                client: &'a C,
                roast_id: &'a uuid::Uuid,
            ) -> RoastQuery<'a, C, Roast, 1> {
                RoastQuery {
                    client,
                    params: [roast_id],
                    stmt: &mut self.0,
                    extractor: |row| Roast {
                        roast_id: row.get(0),
                        bean_id: row.get(1),
                        ts: row.get(2),
                        roast_level_id: row.get(3),
                    },
                    mapper: |it| <Roast>::from(it),
                }
            }
        }
        pub fn insert_roast() -> InsertRoastStmt {
            InsertRoastStmt(cornucopia_async::private::Stmt::new(
                "insert into core.roast (roast_id, bean_id, roast_level_id, ts)
values ($1, $2, $3, $4)
returning roast_id, bean_id, roast_level_id, ts",
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
            ) -> RoastQuery<'a, C, Roast, 4> {
                RoastQuery {
                    client,
                    params: [roast_id, bean_id, roast_level_id, ts],
                    stmt: &mut self.0,
                    extractor: |row| Roast {
                        roast_id: row.get(0),
                        bean_id: row.get(1),
                        ts: row.get(3),
                        roast_level_id: row.get(2),
                    },
                    mapper: |it| <Roast>::from(it),
                }
            }
        }
        impl<'a, C: GenericClient>
            cornucopia_async::Params<'a, InsertRoastParams, RoastQuery<'a, C, Roast, 4>, C>
            for InsertRoastStmt
        {
            fn params(
                &'a mut self,
                client: &'a C,
                params: &'a InsertRoastParams,
            ) -> RoastQuery<'a, C, Roast, 4> {
                self.bind(
                    client,
                    &params.roast_id,
                    &params.bean_id,
                    &params.roast_level_id,
                    &params.ts,
                )
            }
        }
        pub fn update_roast() -> UpdateRoastStmt {
            UpdateRoastStmt(cornucopia_async::private::Stmt::new(
                "update core.roast
set bean_id        = $1,
    roast_level_id = $2,
    ts             = $3
where roast_id = $4
returning roast_id, bean_id, roast_level_id, ts",
            ))
        }
        pub struct UpdateRoastStmt(cornucopia_async::private::Stmt);
        impl UpdateRoastStmt {
            pub fn bind<'a, C: GenericClient>(
                &'a mut self,
                client: &'a C,
                bean_id: &'a uuid::Uuid,
                roast_level_id: &'a uuid::Uuid,
                ts: &'a time::OffsetDateTime,
                roast_id: &'a uuid::Uuid,
            ) -> RoastQuery<'a, C, Roast, 4> {
                RoastQuery {
                    client,
                    params: [bean_id, roast_level_id, ts, roast_id],
                    stmt: &mut self.0,
                    extractor: |row| Roast {
                        roast_id: row.get(0),
                        bean_id: row.get(1),
                        ts: row.get(3),
                        roast_level_id: row.get(2),
                    },
                    mapper: |it| <Roast>::from(it),
                }
            }
        }
        impl<'a, C: GenericClient>
            cornucopia_async::Params<'a, UpdateRoastParams, RoastQuery<'a, C, Roast, 4>, C>
            for UpdateRoastStmt
        {
            fn params(
                &'a mut self,
                client: &'a C,
                params: &'a UpdateRoastParams,
            ) -> RoastQuery<'a, C, Roast, 4> {
                self.bind(
                    client,
                    &params.bean_id,
                    &params.roast_level_id,
                    &params.ts,
                    &params.roast_id,
                )
            }
        }
        pub fn delete_roast() -> DeleteRoastStmt {
            DeleteRoastStmt(cornucopia_async::private::Stmt::new(
                "delete from core.roast
where roast_id = $1",
            ))
        }
        pub struct DeleteRoastStmt(cornucopia_async::private::Stmt);
        impl DeleteRoastStmt {
            pub async fn bind<'a, C: GenericClient>(
                &'a mut self,
                client: &'a C,
                roast_id: &'a uuid::Uuid,
            ) -> Result<u64, tokio_postgres::Error> {
                let stmt = self.0.prepare(client).await?;
                client.execute(stmt, &[roast_id]).await
            }
        }
        pub fn all_roast_levels() -> AllRoastLevelsStmt {
            AllRoastLevelsStmt(cornucopia_async::private::Stmt::new(
                "select roast_level_id,
       name,
       description
from core.roast_level",
            ))
        }
        pub struct AllRoastLevelsStmt(cornucopia_async::private::Stmt);
        impl AllRoastLevelsStmt {
            pub fn bind<'a, C: GenericClient>(
                &'a mut self,
                client: &'a C,
            ) -> RoastLevelQuery<'a, C, RoastLevel, 0> {
                RoastLevelQuery {
                    client,
                    params: [],
                    stmt: &mut self.0,
                    extractor: |row| RoastLevelBorrowed {
                        roast_level_id: row.get(0),
                        name: row.get(1),
                        description: row.get(2),
                    },
                    mapper: |it| <RoastLevel>::from(it),
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
            ) -> RoastStepQuery<'a, C, RoastStep, 1> {
                RoastStepQuery {
                    client,
                    params: [roast_id],
                    stmt: &mut self.0,
                    extractor: |row| RoastStepBorrowed {
                        roast_step_id: row.get(0),
                        roast_id: row.get(1),
                        position: row.get(2),
                        description: row.get(3),
                        time: row.get(4),
                        fan_speed: row.get(5),
                        temp_setting: row.get(6),
                        temperature: row.get(7),
                    },
                    mapper: |it| <RoastStep>::from(it),
                }
            }
        }
        pub fn find_roast_step_by_id() -> FindRoastStepByIdStmt {
            FindRoastStepByIdStmt(cornucopia_async::private::Stmt::new(
                "select roast_step_id,
       roast_id,
       position,
       description,
       time,
       fan_speed,
       temp_setting,
       temperature
from core.roast_step
where roast_step_id = $1",
            ))
        }
        pub struct FindRoastStepByIdStmt(cornucopia_async::private::Stmt);
        impl FindRoastStepByIdStmt {
            pub fn bind<'a, C: GenericClient>(
                &'a mut self,
                client: &'a C,
                roast_step_id: &'a uuid::Uuid,
            ) -> RoastStepQuery<'a, C, RoastStep, 1> {
                RoastStepQuery {
                    client,
                    params: [roast_step_id],
                    stmt: &mut self.0,
                    extractor: |row| RoastStepBorrowed {
                        roast_step_id: row.get(0),
                        roast_id: row.get(1),
                        position: row.get(2),
                        description: row.get(3),
                        time: row.get(4),
                        fan_speed: row.get(5),
                        temp_setting: row.get(6),
                        temperature: row.get(7),
                    },
                    mapper: |it| <RoastStep>::from(it),
                }
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
                description: &'a Option<T1>,
                time: &'a i64,
                fan_speed: &'a Option<i32>,
                temp_setting: &'a Option<i32>,
                temperature: &'a Option<rust_decimal::Decimal>,
            ) -> RoastStepQuery<'a, C, RoastStep, 8> {
                RoastStepQuery {
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
                    extractor: |row| RoastStepBorrowed {
                        roast_step_id: row.get(0),
                        roast_id: row.get(1),
                        position: row.get(2),
                        description: row.get(3),
                        time: row.get(4),
                        fan_speed: row.get(5),
                        temp_setting: row.get(6),
                        temperature: row.get(7),
                    },
                    mapper: |it| <RoastStep>::from(it),
                }
            }
        }
        impl<'a, C: GenericClient, T1: cornucopia_async::StringSql>
            cornucopia_async::Params<
                'a,
                InsertRoastStepParams<T1>,
                RoastStepQuery<'a, C, RoastStep, 8>,
                C,
            > for InsertRoastStepStmt
        {
            fn params(
                &'a mut self,
                client: &'a C,
                params: &'a InsertRoastStepParams<T1>,
            ) -> RoastStepQuery<'a, C, RoastStep, 8> {
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
        pub fn update_roast_step() -> UpdateRoastStepStmt {
            UpdateRoastStepStmt(cornucopia_async::private::Stmt::new(
                "update core.roast_step
set roast_id     = $1,
    position     = $2,
    description  = $3,
    time = $4,
    fan_speed    = $5,
    temp_setting = $6,
    temperature  = $7
where roast_step_id = $8
returning
    roast_step_id,
    roast_id,
    position,
    description,
    time,
    fan_speed,
    temp_setting,
    temperature",
            ))
        }
        pub struct UpdateRoastStepStmt(cornucopia_async::private::Stmt);
        impl UpdateRoastStepStmt {
            pub fn bind<'a, C: GenericClient, T1: cornucopia_async::StringSql>(
                &'a mut self,
                client: &'a C,
                roast_id: &'a uuid::Uuid,
                position: &'a i32,
                description: &'a Option<T1>,
                time: &'a i64,
                fan_speed: &'a Option<i32>,
                temp_setting: &'a Option<i32>,
                temperature: &'a Option<rust_decimal::Decimal>,
                roast_step_id: &'a uuid::Uuid,
            ) -> RoastStepQuery<'a, C, RoastStep, 8> {
                RoastStepQuery {
                    client,
                    params: [
                        roast_id,
                        position,
                        description,
                        time,
                        fan_speed,
                        temp_setting,
                        temperature,
                        roast_step_id,
                    ],
                    stmt: &mut self.0,
                    extractor: |row| RoastStepBorrowed {
                        roast_step_id: row.get(0),
                        roast_id: row.get(1),
                        position: row.get(2),
                        description: row.get(3),
                        time: row.get(4),
                        fan_speed: row.get(5),
                        temp_setting: row.get(6),
                        temperature: row.get(7),
                    },
                    mapper: |it| <RoastStep>::from(it),
                }
            }
        }
        impl<'a, C: GenericClient, T1: cornucopia_async::StringSql>
            cornucopia_async::Params<
                'a,
                UpdateRoastStepParams<T1>,
                RoastStepQuery<'a, C, RoastStep, 8>,
                C,
            > for UpdateRoastStepStmt
        {
            fn params(
                &'a mut self,
                client: &'a C,
                params: &'a UpdateRoastStepParams<T1>,
            ) -> RoastStepQuery<'a, C, RoastStep, 8> {
                self.bind(
                    client,
                    &params.roast_id,
                    &params.position,
                    &params.description,
                    &params.time,
                    &params.fan_speed,
                    &params.temp_setting,
                    &params.temperature,
                    &params.roast_step_id,
                )
            }
        }
        pub fn delete_roast_step() -> DeleteRoastStepStmt {
            DeleteRoastStepStmt(cornucopia_async::private::Stmt::new(
                "delete from core.roast
where roast_id = $1",
            ))
        }
        pub struct DeleteRoastStepStmt(cornucopia_async::private::Stmt);
        impl DeleteRoastStepStmt {
            pub async fn bind<'a, C: GenericClient>(
                &'a mut self,
                client: &'a C,
                roast_id: &'a uuid::Uuid,
            ) -> Result<u64, tokio_postgres::Error> {
                let stmt = self.0.prepare(client).await?;
                client.execute(stmt, &[roast_id]).await
            }
        }
    }
    pub mod roast_level {
        use cornucopia_async::GenericClient;
        use futures;
        use futures::{StreamExt, TryStreamExt};
        #[derive(Clone, Copy, Debug)]
        pub struct InsertRoastWithLevelParams {
            pub roast_id: uuid::Uuid,
            pub bean_id: uuid::Uuid,
            pub roast_level_id: uuid::Uuid,
            pub ts: time::OffsetDateTime,
        }
        #[derive(serde::Serialize, Debug, Clone, PartialEq)]
        pub struct AllRoastsWithLevel {
            pub roast_id: uuid::Uuid,
            pub bean_id: uuid::Uuid,
            pub roast_ts: time::OffsetDateTime,
            pub roast_level_id: uuid::Uuid,
            pub roast_level_name: String,
            pub roast_level_description: String,
        }
        pub struct AllRoastsWithLevelBorrowed<'a> {
            pub roast_id: uuid::Uuid,
            pub bean_id: uuid::Uuid,
            pub roast_ts: time::OffsetDateTime,
            pub roast_level_id: uuid::Uuid,
            pub roast_level_name: &'a str,
            pub roast_level_description: &'a str,
        }
        impl<'a> From<AllRoastsWithLevelBorrowed<'a>> for AllRoastsWithLevel {
            fn from(
                AllRoastsWithLevelBorrowed {
                    roast_id,
                    bean_id,
                    roast_ts,
                    roast_level_id,
                    roast_level_name,
                    roast_level_description,
                }: AllRoastsWithLevelBorrowed<'a>,
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
        pub struct AllRoastsWithLevelQuery<'a, C: GenericClient, T, const N: usize> {
            client: &'a C,
            params: [&'a (dyn postgres_types::ToSql + Sync); N],
            stmt: &'a mut cornucopia_async::private::Stmt,
            extractor: fn(&tokio_postgres::Row) -> AllRoastsWithLevelBorrowed,
            mapper: fn(AllRoastsWithLevelBorrowed) -> T,
        }
        impl<'a, C, T: 'a, const N: usize> AllRoastsWithLevelQuery<'a, C, T, N>
        where
            C: GenericClient,
        {
            pub fn map<R>(
                self,
                mapper: fn(AllRoastsWithLevelBorrowed) -> R,
            ) -> AllRoastsWithLevelQuery<'a, C, R, N> {
                AllRoastsWithLevelQuery {
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
        pub struct FindRoastWithLevelById {
            pub roast_id: uuid::Uuid,
            pub bean_id: uuid::Uuid,
            pub roast_ts: time::OffsetDateTime,
            pub roast_level_id: uuid::Uuid,
            pub roast_level_name: String,
            pub roast_level_description: String,
        }
        pub struct FindRoastWithLevelByIdBorrowed<'a> {
            pub roast_id: uuid::Uuid,
            pub bean_id: uuid::Uuid,
            pub roast_ts: time::OffsetDateTime,
            pub roast_level_id: uuid::Uuid,
            pub roast_level_name: &'a str,
            pub roast_level_description: &'a str,
        }
        impl<'a> From<FindRoastWithLevelByIdBorrowed<'a>> for FindRoastWithLevelById {
            fn from(
                FindRoastWithLevelByIdBorrowed {
                    roast_id,
                    bean_id,
                    roast_ts,
                    roast_level_id,
                    roast_level_name,
                    roast_level_description,
                }: FindRoastWithLevelByIdBorrowed<'a>,
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
        pub struct FindRoastWithLevelByIdQuery<'a, C: GenericClient, T, const N: usize> {
            client: &'a C,
            params: [&'a (dyn postgres_types::ToSql + Sync); N],
            stmt: &'a mut cornucopia_async::private::Stmt,
            extractor: fn(&tokio_postgres::Row) -> FindRoastWithLevelByIdBorrowed,
            mapper: fn(FindRoastWithLevelByIdBorrowed) -> T,
        }
        impl<'a, C, T: 'a, const N: usize> FindRoastWithLevelByIdQuery<'a, C, T, N>
        where
            C: GenericClient,
        {
            pub fn map<R>(
                self,
                mapper: fn(FindRoastWithLevelByIdBorrowed) -> R,
            ) -> FindRoastWithLevelByIdQuery<'a, C, R, N> {
                FindRoastWithLevelByIdQuery {
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
        pub struct InsertRoastWithLevel {
            pub roast_id: uuid::Uuid,
            pub bean_id: uuid::Uuid,
            pub roast_ts: time::OffsetDateTime,
            pub roast_level_id: uuid::Uuid,
            pub roast_level_name: String,
            pub roast_level_description: String,
        }
        pub struct InsertRoastWithLevelBorrowed<'a> {
            pub roast_id: uuid::Uuid,
            pub bean_id: uuid::Uuid,
            pub roast_ts: time::OffsetDateTime,
            pub roast_level_id: uuid::Uuid,
            pub roast_level_name: &'a str,
            pub roast_level_description: &'a str,
        }
        impl<'a> From<InsertRoastWithLevelBorrowed<'a>> for InsertRoastWithLevel {
            fn from(
                InsertRoastWithLevelBorrowed {
                    roast_id,
                    bean_id,
                    roast_ts,
                    roast_level_id,
                    roast_level_name,
                    roast_level_description,
                }: InsertRoastWithLevelBorrowed<'a>,
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
        pub struct InsertRoastWithLevelQuery<'a, C: GenericClient, T, const N: usize> {
            client: &'a C,
            params: [&'a (dyn postgres_types::ToSql + Sync); N],
            stmt: &'a mut cornucopia_async::private::Stmt,
            extractor: fn(&tokio_postgres::Row) -> InsertRoastWithLevelBorrowed,
            mapper: fn(InsertRoastWithLevelBorrowed) -> T,
        }
        impl<'a, C, T: 'a, const N: usize> InsertRoastWithLevelQuery<'a, C, T, N>
        where
            C: GenericClient,
        {
            pub fn map<R>(
                self,
                mapper: fn(InsertRoastWithLevelBorrowed) -> R,
            ) -> InsertRoastWithLevelQuery<'a, C, R, N> {
                InsertRoastWithLevelQuery {
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
        pub fn all_roasts_with_level() -> AllRoastsWithLevelStmt {
            AllRoastsWithLevelStmt(cornucopia_async::private::Stmt::new(
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
        pub struct AllRoastsWithLevelStmt(cornucopia_async::private::Stmt);
        impl AllRoastsWithLevelStmt {
            pub fn bind<'a, C: GenericClient>(
                &'a mut self,
                client: &'a C,
            ) -> AllRoastsWithLevelQuery<'a, C, AllRoastsWithLevel, 0> {
                AllRoastsWithLevelQuery {
                    client,
                    params: [],
                    stmt: &mut self.0,
                    extractor: |row| AllRoastsWithLevelBorrowed {
                        roast_id: row.get(0),
                        bean_id: row.get(1),
                        roast_ts: row.get(2),
                        roast_level_id: row.get(3),
                        roast_level_name: row.get(4),
                        roast_level_description: row.get(5),
                    },
                    mapper: |it| <AllRoastsWithLevel>::from(it),
                }
            }
        }
        pub fn find_roast_with_level_by_id() -> FindRoastWithLevelByIdStmt {
            FindRoastWithLevelByIdStmt(cornucopia_async::private::Stmt::new(
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
        pub struct FindRoastWithLevelByIdStmt(cornucopia_async::private::Stmt);
        impl FindRoastWithLevelByIdStmt {
            pub fn bind<'a, C: GenericClient>(
                &'a mut self,
                client: &'a C,
                roast_id: &'a uuid::Uuid,
            ) -> FindRoastWithLevelByIdQuery<'a, C, FindRoastWithLevelById, 1> {
                FindRoastWithLevelByIdQuery {
                    client,
                    params: [roast_id],
                    stmt: &mut self.0,
                    extractor: |row| FindRoastWithLevelByIdBorrowed {
                        roast_id: row.get(0),
                        bean_id: row.get(1),
                        roast_ts: row.get(2),
                        roast_level_id: row.get(3),
                        roast_level_name: row.get(4),
                        roast_level_description: row.get(5),
                    },
                    mapper: |it| <FindRoastWithLevelById>::from(it),
                }
            }
        }
        pub fn insert_roast_with_level() -> InsertRoastWithLevelStmt {
            InsertRoastWithLevelStmt(cornucopia_async::private::Stmt::new(
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
        pub struct InsertRoastWithLevelStmt(cornucopia_async::private::Stmt);
        impl InsertRoastWithLevelStmt {
            pub fn bind<'a, C: GenericClient>(
                &'a mut self,
                client: &'a C,
                roast_id: &'a uuid::Uuid,
                bean_id: &'a uuid::Uuid,
                roast_level_id: &'a uuid::Uuid,
                ts: &'a time::OffsetDateTime,
            ) -> InsertRoastWithLevelQuery<'a, C, InsertRoastWithLevel, 4> {
                InsertRoastWithLevelQuery {
                    client,
                    params: [roast_id, bean_id, roast_level_id, ts],
                    stmt: &mut self.0,
                    extractor: |row| InsertRoastWithLevelBorrowed {
                        roast_id: row.get(0),
                        bean_id: row.get(1),
                        roast_ts: row.get(2),
                        roast_level_id: row.get(3),
                        roast_level_name: row.get(4),
                        roast_level_description: row.get(5),
                    },
                    mapper: |it| <InsertRoastWithLevel>::from(it),
                }
            }
        }
        impl<'a, C: GenericClient>
            cornucopia_async::Params<
                'a,
                InsertRoastWithLevelParams,
                InsertRoastWithLevelQuery<'a, C, InsertRoastWithLevel, 4>,
                C,
            > for InsertRoastWithLevelStmt
        {
            fn params(
                &'a mut self,
                client: &'a C,
                params: &'a InsertRoastWithLevelParams,
            ) -> InsertRoastWithLevelQuery<'a, C, InsertRoastWithLevel, 4> {
                self.bind(
                    client,
                    &params.roast_id,
                    &params.bean_id,
                    &params.roast_level_id,
                    &params.ts,
                )
            }
        }
    }
}
