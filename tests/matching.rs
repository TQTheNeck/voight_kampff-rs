use voight_kampff;

#[test]
fn bot() {
    assert_eq!(voight_kampff::bot("DoCoMo/2.0 N905i(c100;TB;W24H16) (compatible; Googlebot-Mobile/2.1; +http://www.google.com/bot.html)"), true);
}

#[test]
fn not_bot() {
    assert_eq!(voight_kampff::bot("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/87.0.4280.66 Safari/537.36"), false);
}
