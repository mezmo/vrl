// https://www.oreilly.com/library/view/regular-expressions-cookbook/9781449327453/ch04s12.html
// (converted to non-lookaround version given `regex` does not support lookarounds)
// See also: https://www.ssa.gov/history/ssn/geocard.html
pub(crate) const US_SOCIAL_SECURITY_NUMBER_PATTERN: &str = r#"(?x)                                                               # Ignore whitespace and comments in the regex expression.
    \b(?:00[1-9]|0[1-9][0-9]|[1-578][0-9]{2}|6[0-57-9][0-9]|66[0-57-9])[-\s]    # Area number: 001-899 except 666
    (?:0[1-9]|[1-9]0|[1-9][1-9])[-\s]                                         # Group number: 01-99
    (?:000[1-9]|00[1-9]0|0[1-9]00|[1-9]000|[1-9]{4})\b                      # Serial number: 0001-9999
    "#;
// Patterns taken from: https://github.com/logdna/logdna-agent-v2/blob/master/docs/REGEX.md
pub(crate) const EMAIL_ADDRESS_PATTERN: &str = r#"(?x)
    (?i:[a-z0-9!\#$%&'*+/=?^_`{|}~-]+(?:\.[a-z0-9!\#$%&'*+/=?^_`{|}~-]+)*|"(?:[\x01-\x08\x0b\x0c\x0e-\x1f\x21\x23-\x5b\x5d-\x7f]|\\[\x01-\x09\x0b\x0c\x0e-\x7f])*")@(?:(?:[a-z0-9](?:[a-z0-9-]*[a-z0-9])?\.)+[a-z0-9](?:[a-z0-9-]*[a-z0-9])?|\[(?:(?:(2(5[0-5]|[0-4][0-9])|1[0-9][0-9]|[1-9]?[0-9]))\.){3}(?:(2(5[0-5]|[0-4][0-9])|1[0-9][0-9]|[1-9]?[0-9])|[a-z0-9-]*[a-z0-9]:(?:[\x01-\x08\x0b\x0c\x0e-\x1f\x21-\x5a\x53-\x7f]|\\[\x01-\x09\x0b\x0c\x0e-\x7f])+)\])
    "#;
// See: https://docs.trellix.com/bundle/data-loss-prevention-11.10.x-classification-definitions-reference-guide/page/GUID-B8D29ECE-E70A-401E-B18D-B773F4FF71ED.html
// And: https://baymard.com/checkout-usability/credit-card-patterns
pub(crate) const CREDIT_CARD_PATTERN: &str = r#"(?x)
    \b
    (?:4\d{3}[-\s]?\d{4}[-\s]?\d{4}[-\s]?\d{4}|4\d{12})|                # visa cards, 13 or 16 digits starting with 4. format pattern for 13 digit is unknown  
    (?:(?:2[2-7]|5[1-7])\d{2}[-\s]?\d{4}[-\s]?\d{4}[-\s]?\d{4})|        # mastercard, 16 digits. old range: 22-27, new range: 51-57 
    (?:(?:2131|1800|35\d{2})[-\s]?\d{4}[-\s]?\d{4}[-\s]?\d{4})|         # JCB cards, 16 digits (starts with 2131, 1800 or 35)
    (?:(?:6011|64[4-9]\d|65\d{2})[-\s]?\d{4}[-\s]?\d{4}[-\s]?\d{4})|    # Discover cards, 16 digits (starts with 6011, 644, 645, 646, 647, 648, 649 or 65)
    (?:3[47]\d{2}[-\s]?\d{6}[-\s]?\d{5})|                               # AMEX cards, 15 digits starting with 34 or 37
    (?:(?:30[0-5]\d|309\d|3[689]\d{2})[-\s]?\d{6}[-\s]?\d{4})           # Diner's club, 14 digits, starting with 300-305 or 309 or 36,38,39
    \b
"#;
pub(crate) const IPV4_ADDRESS_PATTERN: &str = r#"(?x)
        \b(?:(?:25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?)\.){3}   # first 3 octets (0.0.0.) with trailing period
        (?:25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?)\b             # last octet
        "#;
pub(crate) const PHONE_NUMBER_PATTERN: &str = r#"(?x)
        (?:
            (?:                                 # optional country code
                (?:\+\d{1,3}|\b\d{1,3})[\s.-]?  # or used to prevent \b from consuming leading +
            )?      
            \(?\d{3}\)?[\s.-]?                  # optional area code
            \d{3}[\s.-]?                        # first 3 digits of phone
            \d{4}                               # last 4 digits
        )\b     
        "#;
