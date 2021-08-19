use crate::domain::model::user::{NewUser, User};
use anyhow::Result;

pub trait UserRepository {
    /// DBから指定されたidのユーザを取得する。
    /// ユーザが存在しなければErr(NotFound)が返る。
    fn get_user(&self, id: usize) -> Result<User>;

    /// DBからユーザを全て取得する。
    fn get_users(&self) -> Result<Vec<User>>;

    /// ユーザを新しく作成する。
    fn create_user(&self, user: &NewUser) -> Result<()>;

    /// ユーザ情報を更新する。
    /// 更新に成功すればそのユーザのエントリを返す。
    fn update_user(&self, user: &User) -> Result<User>;

    /// ユーザを削除する.
    fn delete_user(&self, user: &User) -> Result<()>;
}
