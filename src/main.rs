use teloxide::prelude::*;

async fn send_request(link: String) -> Result<String, reqwest::Error> {
    let mut url = String::new(); // делаем пустую строку
    if !link.starts_with("https://") {
        url = format!("https://{}", &link); // добавляем https:// к исходной ссылке
    } else if link.starts_with("http://") {
        url = format!("https://{}", &link[6..].to_string()); // добавляем https:// к исходной ссылке
    } else { url = link; };
    let body = reqwest::get(url) // делаем запрос
        .await?
        .text()
        .await?;
    Ok(body) // возвращаем ответ (код страницы)
}

const BAD_LINKS: [&str; 16] = [ // плохие ссылки с логгером (16 штук, статичный список)
    "iplogger.org",
    "wl.gl",
    "ed.tc",
    "bc.ax",
    "iplogger.com",
    "maper.info",
    "iplogger.ru",
    "2no.co",
    "yip.su",
    "iplogger.info",
    "iplis.ru",
    "ezstat.ru",
    "iplog.co",
    "iplogger.cn",
    "grabify.link",
    "hueglotik.lol"
];

#[tokio::main]
async fn main() {
    let bot = Bot::new("").auto_send(); // инициализируем бота

    teloxide::repl(bot, |message: Message, bot: AutoSend<Bot>| async move { // хендлер при сообщениях

        // треугольник 
        if let Some(text) = message.text() {
            for word in text.split_whitespace() { // раздробляем по пробелам строку на список
                if word.starts_with("telegra.ph") || word.starts_with("https://telegra.ph") || word.starts_with("http://telegra.ph") { // проверка на наличие telegra.ph в сообщении
                    let _ = bot.send_message(message.chat.id, "⌛️ Проверяем telegraph статью на Logger").reply_to_message_id(message.id).await;
                    if let Ok(str) = send_request(word.to_string()).await { // получаем исходный код страницы
                        let re = regex::Regex::new(r#"<img src="([^"]+)""#).unwrap(); // ищет картинки
                        for cap in re.captures_iter(str.as_str()) {
                            for link in &BAD_LINKS {
                                if cap[1].contains(link) { // перебираем плохие ссылки и ищем соответствие
                                    let _ = bot.send_message(message.chat.id, "❌ Telegraph статья содержит Logger").reply_to_message_id(message.id).await;
                                    let _ = bot.delete_message(message.chat.id, message.id).await;
                                    return respond(());
                                };
                            };
                        };
                        let _ = bot.send_message(message.chat.id, "✅ Telegraph статья не содержит Logger").reply_to_message_id(message.id).await;
                    } else { let _ = bot.send_message(message.chat.id, "❌ Не удалось проверить telegraph статью на Logger").reply_to_message_id(message.id).await;  }
                };
            };
        };
        respond(())
    }).await;
}
