struct Solution;

use std::hash::{Hash, Hasher};

struct EmailAddress(String);

impl EmailAddress {
    fn normalize(&self) -> String {
        let &EmailAddress(ref addr) = self;
        let domain = addr.rsplit("@").next().unwrap();
        let mut s = addr
            .split("@")
            .next()
            .unwrap()
            .split("+")
            .next()
            .unwrap()
            .replace(".", "");
        s.push('@');
        s.push_str(domain);
        s
    }
}

impl PartialEq for EmailAddress {
    fn eq(&self, other: &Self) -> bool {
        self.normalize() == other.normalize()
    }
}

impl Eq for EmailAddress {}

impl Hash for EmailAddress {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.normalize().hash(state);
    }
}

impl Solution {
    pub fn num_unique_emails(emails: Vec<String>) -> i32 {
        use std::collections::HashSet;
        let mut set = HashSet::new();
        for e in emails {
            set.insert(EmailAddress(e));
        }
        set.len() as i32
    }
}

fn main() {
    ()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_email_address() {
        assert!(
            EmailAddress("test.email+alex@leetcode.com".to_string())
                == EmailAddress("test.e.mail+bob.cathy@leetcode.com".to_string())
        );
        assert!(
            EmailAddress("test.email+alex@lee.tcode.com".to_string())
                != EmailAddress("test.e.mail+bob.cathy@leetcode.com".to_string())
        );
        assert!(
            EmailAddress("test.email+alex@leetcode.com".to_string())
                != EmailAddress("test.email.leet+alex@code.com".to_string())
        );
    }

    #[test]
    fn test_num_unique_emails() {
        assert_eq!(
            Solution::num_unique_emails(vec![
                "test.email+alex@leetcode.com".to_string(),
                "test.e.mail+bob.cathy@leetcode.com".to_string(),
                "testemail+david@lee.tcode.com".to_string(),
                "t.es.temail+da.vi+d@lee.tcode.com".to_string()
            ]),
            2
        );

        assert_eq!(
            Solution::num_unique_emails(vec![
                "test.email+alex@leetcode.com".to_string(),
                "test.email.leet+alex@code.com".to_string(),
            ]),
            2
        );
    }
}
