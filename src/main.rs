use teloxide::{filter_command, handler, prelude::*, repl, utils::command::BotCommands};
use dotenvy::dotenv;
use teloxide::types::Message;
use teloxide::types::AllowedUpdate;

#[tokio::main]
async fn main() {
    pretty_env_logger::init();
    log::info!("Starting command bot...");

    let token = dotenvy::var("TELOXIDE_TOKEN").unwrap(); //Это не трогать
    let bot = Bot::new(token); //Не трогать

    Command::repl(bot, answer).await;

    }


#[derive(BotCommands, Clone)]
#[command(rename_rule = "lowercase", description = "Существуют следующие команды:")]
enum Command {
    #[command(description = "Выводит этот список команд")]
    Help,
    #[command(description = "Ссылка на сайт ВУЗа")]
    Mtuci,
    #[command(description = "Рассказывает о ВУЗе если вы только собрались поступить к нам")]
    Абитуриенту,
    #[command(description = "Мы рассказываем о некоторых наших направлениях")]
    Направления
}

async fn answer(bot: Bot, msg: Message, cmd: Command) -> ResponseResult<()> {
    match cmd {
        Command::Help => {
            let result = bot.send_message(msg.chat.id, Command::descriptions().to_string()).await;
            if let Err(err) = result {
                log::error!("Failed to send message: {:?}", err);
            }
        }
        Command::Mtuci => {
            let result = bot.send_message(msg.chat.id, format!("https://mtuci.ru/")).await;
            if let Err(err) = result {
                log::error!("Failed to send message: {:?}", err);
            }
        }
        Command::Абитуриенту => {
            let result = bot.send_message(msg.chat.id, format!("Информация о месте нахождения образовательной организации:
Адрес МТУСИ: 111024, г. Москва, ул. Авиамоторная, 8а;
Адрес МТУСИ: 123423, г. Москва, ул. Народного Ополчения, д. 32
Адрес КТ МТУСИ: 125493, г. Москва, ул. Авангардная, д. 5.


Информация о режиме и графике работы образовательной организации:
График работы структурных подразделений:

Пн – чт : 09:00- 18:00 ( обед 13:00-14:00)
Пт : 09:00-16:45 ( обед 13:00-14:00)
Прием ректором студентов проводится по понедельникам с 14:00 до 14:30

Информация о контактных телефонах образовательной организации:
Ректорат +7 (495)957-79-17
Пост охраны +7(495)957-7945

Отдел документационного обеспечения управления +7(495)957-7731, факс +7(495)925-04-35

Информация об адресах электронной почты образовательной организации:
Ректорат: mtuci@mtuci.ru
Отдел документационного обеспечения управления: kanc@mtuci.ru")).await;
            if let Err(err) = result {
                log::error!("Failed to send message: {:?}", err);
            }
        }
        Command::Направления => {
            let result = bot.send_message(msg.chat.id, format!("\

            01.03.02 Прикладная математика и информатика
Подготовка специалистов в области математики, анализа данных, программирования, информационно-коммуникационных технологий

01.03.04 Прикладная математика
Подготовка специалистов для решения задач в различных прикладных областях с использованием математики и компьютерных технологий.

02.03.02 Фундаментальная информатика и информационные технологии
Универсальное IT-направление готовит компьютерщиков широкого профиля.

09.03.01 Информатика и вычислительная техника
Направление подготовки в области информационных систем и сетей, разработкой программного обеспечения автоматизированных систем.

09.03.02 Информационные системы и технологии
Это направление обучения в области проектирования, отладки, производства и эксплуатации информационных технологий и систем.

09.03.03 Прикладная информатика
Подготовка IT-специалистов для системного анализа прикладной области, решения прикладных задач информационных систем.

09.03.04 Программная инженерия
Это направление подготовки специалистов для проектирования, разработки и сопровождения программного обеспечения.

И многие другие (Смотрите на официальном сайте)
")).await;
            if let Err(err) = result {
                log::error!("Failed to send message: {:?}", err);

            }
        }
    }

    Ok(())
}


//другой вариант
// async fn handle_message(bot: Bot, msg: Message, text: &str) -> Result<(), Box<dyn std::error::Error>> {
//     if text.contains("привет") {
//         bot.send_message(msg.chat.id, "Привет!").await?;
//     } else if text.contains("как дела?") {
//         bot.send_message(msg.chat.id, "Хорошо").await?;
//     } else {
//         bot.send_message(msg.chat.id, "Я не понимаю вас.").await?;
//     }
//
//     Ok(())
// }
//
// async fn analyze_message(bot: Bot, msg: Message, text: Option<String>) -> Result<(), Box<dyn std::error::Error>> {
//     if let Some(text) = text {
//         handle_message(bot, msg, &text).await?;
//     }
//
//     Ok(())
//}


async fn analyze_message(bot: Bot, msg: Message, text: &str) -> Result<(), Box<dyn std::error::Error>> {
    if text.contains("привет") {
        bot.send_message(msg.chat.id, "Привет!").await?;
    } else if text.contains("как дела?") {
        bot.send_message(msg.chat.id, "Хорошо").await?;
    } else {
        bot.send_message(msg.chat.id, "Я не понимаю вас.").await?;
    }

    Ok(())
}
