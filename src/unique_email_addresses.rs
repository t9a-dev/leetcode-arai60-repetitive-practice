use std::collections::HashSet;

pub struct EmailAddress {
    value: String,
}
impl EmailAddress {
    pub fn new(value: String) -> Result<Self, ()> {
        Ok(Self {
            value: Self::sanitize(value)?,
        })
    }

    pub fn value(self) -> String {
        self.value
    }

    fn sanitize(value: String) -> Result<String, ()> {
        let Some((local, domain)) = value.split_once('@') else {
            return Err(());
        };
        let plus_skipped_local = match local.split_once('+') {
            Some((plus_skipped_local, _)) => plus_skipped_local,
            None => local,
        };
        let sanitized_local = plus_skipped_local.replace(".", "");

        Ok(format!("{sanitized_local}@{domain}"))
    }
}

struct UniqueEmailAddresses {}
impl UniqueEmailAddresses {
    /*
    問題の理解:
    文字列からなる配列emailsについて、emails[i]にメールを送信する場合実際にメールを受信することなるアドレスの数を返す。
    有効なメールアドレスのルール
        - ローカル名とドメイン名で@で区切られている。
        - アルファベット小文字と'.', '+'が含まれる場合がある。
        - ローカル名に'.'が含まれる時、ローカル名に'.' が含まれないメールと同じとみなす。このルールはドメイン名には適用されない。
        - ローカル名に'+'が含まれる時、'+'以降のローカル名は無視される。このルールはドメイン名には適用されない。

    Accepted

    入力自体やドメイン部分のバリデーションを行っていないが、解ける問題に時間をかけても仕方ないのでここまで
    */
    pub fn num_unique_emails(emails: Vec<String>) -> i32 {
        HashSet::<_>::from_iter(
            emails
                .into_iter()
                .flat_map(EmailAddress::new)
                .map(|email_address| email_address.value()),
        )
        .iter()
        .len() as i32
    }
}
