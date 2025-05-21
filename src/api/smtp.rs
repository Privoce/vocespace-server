use lettre::{
    message::header::ContentType, transport::smtp::authentication::Credentials, Message,
    SmtpTransport, Transport,
};

const SMTP_PWD: &str = "ajsb gjun rgvj mvev";
const SMTP_HOST: &str = "smtp.gmail.com";
// use `lettre` crate to send email(smtp)
pub fn send_email(from: &str, cc: &str, to: &str, subject: &str, content: &str) -> bool {
    // get pwd
    let cred = Credentials::new(from.to_string(), SMTP_PWD.to_string());

    let mailer = SmtpTransport::relay(SMTP_HOST)
        .unwrap()
        .credentials(cred)
        .build();
    if cc.is_empty() {
        let email = Message::builder()
            .from(from.parse().unwrap())
            .to(to.parse().unwrap())
            .subject(subject)
            .header(ContentType::TEXT_HTML)
            .body(String::from(content))
            .unwrap();
        // Send
        return mailer.send(&email).is_ok();
    } else {
        let email = Message::builder()
            .from(from.parse().unwrap())
            .reply_to(cc.parse().unwrap())
            .to(to.parse().unwrap())
            .subject(subject)
            .header(ContentType::TEXT_HTML)
            .body(String::from(content))
            .unwrap();
        // Send
        return mailer.send(&email).is_ok();
    }
}

const SMTP_CONTENT_PUY: &str = r##"
<!DOCTYPE html>
<html lang="en">
  <head>
    <meta charset="UTF-8" />
    <meta name="viewport" content="width=device-width, initial-scale=1.0" />
    <title>vocespace</title>
  </head>
  <body>
    <table data-v-57b2d1f5="">
      <tbody data-v-57b2d1f5="">
        <tr data-v-57b2d1f5="">
          <td
            data-v-57b2d1f5=""
            style="
              width: 660px;
              min-width: 660px;
              line-height: 0pt;
              padding: 0px;
              margin: 0px;
              font-weight: normal;
            "
          >
            <table
              data-v-57b2d1f5=""
              width="100%"
              cellspacing="0"
              cellpadding="0"
              border="0"
            >
              <tbody data-v-57b2d1f5="">
                <tr data-v-57b2d1f5="">
                  <td
                    data-v-57b2d1f5=""
                    bgcolor="#212429"
                    class="el-tooltip__trigger"
                    style="padding: 50px"
                  >
                    <table
                      data-v-57b2d1f5=""
                      width="100%"
                      cellspacing="0"
                      cellpadding="0"
                      border="0"
                    >
                      <tbody
                        data-v-57b2d1f5=""
                        style="display: flex; flex-direction: column"
                      >
                        <tr
                          data-v-57b2d1f5=""
                          class="el-tooltip__trigger"
                          style="border: 0px"
                        >
                          <td
                            data-v-57b2d1f5=""
                            style="
                              background-color: rgb(33, 36, 41);
                              text-align: center;
                              display: flex;
                              align-items: center;
                              justify-content: center;
                              flex-direction: column;
                            "
                          >
                            <img
                              data-v-57b2d1f5=""
                              src="data:image/svg+xml;base64,PHN2ZyB3aWR0aD0iMTM1IiBoZWlnaHQ9IjMyIiB2aWV3Qm94PSIwIDAgMTM1IDMyIiBmaWxsPSJub25lIiB4bWxucz0iaHR0cDovL3d3dy53My5vcmcvMjAwMC9zdmciPgo8cGF0aCBmaWxsLXJ1bGU9ImV2ZW5vZGQiIGNsaXAtcnVsZT0iZXZlbm9kZCIgZD0iTTI5LjI4ODQgMjAuNDE4MkMyOS43NDk5IDE5LjAyOTMgMjkuOTk5OCAxNy41NDM4IDI5Ljk5OTggMTZDMjkuOTk5OCA4LjI2ODA2IDIzLjczMTkgMi4wMDAwOCAxNS45OTk5IDIuMDAwMDhDOC4yNjc5OCAyLjAwMDA4IDIgOC4yNjgwNiAyIDE2QzIgMTcuNTc0NCAyLjI1OTg3IDE5LjA4OCAyLjczOTA2IDIwLjUwMDVDMy4wOTg2MSAyMC41MDg4IDMuNDM4MzYgMjAuNTI0NiAzLjc0NDIyIDIwLjUzOUM0LjE5OTkgMjAuNTYwMyA0LjU1Njc4IDIwLjU3NjggNC44NDY1IDIwLjU3MUw0Ljg1NzY0IDIwLjU3MDdMNC45Njg1IDE4LjMyODhMNC45Njk1OSAxOC4zMDY3TDQuOTcxNTUgMTguMjg0N0w1LjA5MDY2IDE2Ljk0NjVMNS4wOTQ3IDE2LjkwMTFMNS4xMDI0IDE2Ljg1NjJMNS4yMzY4NCAxNi4wNzI1QzUuMzc2IDE1LjI2MTMgNS43NjY1MyAxNC41MTQ0IDYuMzUzMjYgMTMuOTM3MkM2Ljg1MSAxMy40NDc1IDcuNDQ2ODggMTMuMTE4MyA4LjA3NjkzIDEyLjk0ODdDNy40MzAyNiAxMi42MDg3IDYuOTg5MjYgMTEuOTMwNCA2Ljk4OTI2IDExLjE0OTFDNi45ODkyNiAxMC4wMjcgNy44OTg5IDkuMTE3MzcgOS4wMjEgOS4xMTczN0MxMC4xNDMxIDkuMTE3MzcgMTEuMDUyNyAxMC4wMjcgMTEuMDUyNyAxMS4xNDkxQzExLjA1MjcgMTEuOTA5MSAxMC42MzU1IDEyLjU3MTcgMTAuMDE3NSAxMi45MjAxQzEwLjU1ODQgMTMuMDQ4NCAxMS4wNzczIDEzLjI5MjEgMTEuNTM0NSAxMy42NTA2QzExLjczNzEgMTMuMzQ2NCAxMS45NzE0IDEzLjA2MjIgMTIuMjM0NyAxMi44MDMxQzEzLjAyMDkgMTIuMDI5NiAxMy45ODczIDExLjU1IDE0Ljk5NzkgMTEuMzYyNkMxNC4xMzU0IDEwLjk4MjcgMTMuNTMzMiAxMC4xMjA0IDEzLjUzMzIgOS4xMTc0MUMxMy41MzMyIDcuNzYzMTUgMTQuNjMxIDYuNjY1MyAxNS45ODUzIDYuNjY1M0MxNy4zMzk2IDYuNjY1MyAxOC40Mzc0IDcuNzYzMTUgMTguNDM3NCA5LjExNzQxQzE4LjQzNzQgMTAuMTIxNCAxNy44MzQgMTAuOTg0NSAxNi45NzAxIDExLjM2MzhDMTcuOTcxNiAxMS41NTA0IDE4LjkzMDEgMTIuMDIyNyAxOS43MTMzIDEyLjc3ODlDMTkuOTg0NCAxMy4wNDA3IDIwLjIyNjEgMTMuMzI5MyAyMC40MzU0IDEzLjYzOTFDMjAuODgzNiAxMy4yNTE5IDIxLjM5OTIgMTIuOTgxOCAyMS45NDEyIDEyLjgyOTZDMjEuMzI5NSAxMi40Nzk0IDIwLjkxNzMgMTEuODIwNCAyMC45MTczIDExLjA2NUMyMC45MTczIDkuOTQyOTUgMjEuODI3IDkuMDMzMyAyMi45NDkxIDkuMDMzM0MyNC4wNzEyIDkuMDMzMyAyNC45ODA4IDkuOTQyOTUgMjQuOTgwOCAxMS4wNjVDMjQuOTgwOCAxMS44MDQxIDI0LjU4NjIgMTIuNDUxMSAyMy45OTYxIDEyLjgwNjZDMjQuNjQxNiAxMi45NzI4IDI1LjI1MjkgMTMuMzA1NiAyNS43NjE3IDEzLjgwNjFDMjYuMzQ4NSAxNC4zODMzIDI2LjczOSAxNS4xMzAzIDI2Ljg3ODIgMTUuOTQxNUwyNy4wMTI2IDE2LjcyNTJMMjcuMDIwNiAxNi43NzE3TDI3LjAyNDYgMTYuODE4N0wyNy4xODU4IDE4LjY4OTNMMjcuMTg2NiAxOC42OTkzTDI3LjE4NzMgMTguNzA5NEwyNy4zMDQ0IDIwLjQzMDRDMjcuMzM3IDIwLjQzMjYgMjcuMzcyMyAyMC40MzQ2IDI3LjQxMDQgMjAuNDM2NEMyNy43OTYxIDIwLjQ1NDMgMjguMjMwNyAyMC40NDM1IDI4Ljc2MTcgMjAuNDMwNEMyOC45Mjc0IDIwLjQyNjMgMjkuMTAyNSAyMC40MjIgMjkuMjg4NCAyMC40MTgyWk0yOC4zMDI4IDIyLjY4NjZDMjcuOTI4MSAyMi42OTI1IDI3LjU4NiAyMi42OTMzIDI3LjMwNjQgMjIuNjgwM0MyNi45MzM0IDIyLjY2MyAyNi40NzYgMjIuNjE5NSAyNi4wODM4IDIyLjQ1MjdDMjUuODcyOCAyMi4zNjI5IDI1LjYxNzIgMjIuMjExNSAyNS40MTM4IDIxLjk0OTdDMjUuMjQzMiAyMS43MzAxIDI1LjE0NzQgMjEuNDgxNiAyNS4xMTQyIDIxLjIyOTVMMjUuMTA3MyAyMS4yM0wyNC45NDY4IDE4Ljg3MkwyNC43OTA2IDE3LjA1ODVMMjQuNjY0MiAxNi4zMjEzQzI0LjYwNDYgMTUuOTc0MiAyNC40Mzc1IDE1LjY1NDUgMjQuMTg2NCAxNS40MDc1QzIzLjU0MDUgMTQuNzcyMSAyMi40OTI1IDE0Ljc2NjkgMjEuODMzIDE1LjQwMzZDMjEuNjAxNiAxNS42MjcxIDIxLjQzODEgMTUuOTEyMiAyMS4zNjE5IDE2LjIyMTNMMjEuNDgwMSAxNi45MTZMMjEuNDgyNCAxNi45M0wyMS40ODQ1IDE2Ljk0NDFMMjEuNzE2OCAxOC41NTUzTDIxLjcxNzMgMTguNTU4N0wyMS44ODkyIDE5Ljc3NzdMMjEuOTAwMiAxOS44NTU4VjE5LjkzNDZWMjEuMTI1N0gyMS45MDA0QzIxLjkwMDQgMjEuNjAwOCAyMS43MzAzIDIyLjA2NjcgMjEuMzYyNiAyMi40MDUxQzIxLjAxMDcgMjIuNzI4OSAyMC41NjkzIDIyLjg1MDkgMjAuMTc0NyAyMi44MzYxQzE5LjM5NzYgMjIuODA3IDE4LjU0ODEgMjIuMjA2NSAxOC41MzMzIDIxLjE1MzdIMTguNTMzMVYxOS45OTA3VjE5LjkxMjRMMTguNTQzOSAxOS44MzQ4TDE4Ljc1NDEgMTguMzM1NUwxOC43NTQxIDE4LjMzNTVMMTguNzU1MyAxOC4zMjdMMTguOTg2NSAxNi43NjQ3TDE4Ljk4ODMgMTYuNzUyN0wxOC45OTAzIDE2Ljc0MDdMMTkuMDgxIDE2LjIwNzlMMTkuMDY2NCAxNi4xMjJDMTguOTU0OSAxNS40NjY2IDE4LjYzMzEgMTQuODU4NiAxOC4xNTI5IDE0LjM5NDhDMTYuOTM4NiAxMy4yMjIzIDE1LjAwNTkgMTMuMjI3OSAxMy44MTAyIDE0LjQwNDNDMTMuMzg2MSAxNC44MjE2IDEzLjA5MTggMTUuMzUxMSAxMi45NjA2IDE1LjkyOTFDMTIuOTcyNCAxNS45ODQyIDEyLjk4MzEgMTYuMDM5NiAxMi45OTI2IDE2LjA5NTNMMTMuMTI0NiAxNi44NzE3TDEzLjEzMTQgMTYuOTExM0wxMy4xMzUzIDE2Ljk1MTNMMTMuMjg3NiAxOC41MTY1TDEzLjQ2MDQgMTkuODQzNkwxMy40Njk4IDE5LjkxNThWMTkuOTg4NlYyMS4yNTE4SDEzLjQ3MjJDMTMuNDcyMiAyMS43MjQ3IDEzLjMwNDEgMjIuMTg5OSAxMi45MzggMjIuNTI4OEMxMi41ODcgMjIuODUzOCAxMi4xNDU1IDIyLjk3NzIgMTEuNzQ5NyAyMi45NjIzQzEwLjk2OCAyMi45MzI4IDEwLjEyNjggMjIuMzI2NyAxMC4xMTIxIDIxLjI3OThIMTAuMTA0N1YxOS45NDg2VjE5Ljg3MjRMMTAuMTE1IDE5Ljc5NjlMMTAuMjgzMSAxOC41NjM5TDEwLjI4MzEgMTguNTYzOEwxMC41MDU3IDE2LjkzMTZMMTAuNTA4MyAxNi45MTI0TDEwLjUxMTUgMTYuODkzNEwxMC42NTM4IDE2LjA2NDFDMTAuNTY1MyAxNS44NjcgMTAuNDM5MyAxNS42ODY2IDEwLjI4MTkgMTUuNTM0N0M5LjYyMjUxIDE0Ljg5OCA4LjU3NDQ5IDE0LjkwMzEgNy45Mjg2MiAxNS41Mzg1QzcuNjc3NTIgMTUuNzg1NSA3LjUxMDM5IDE2LjEwNTIgNy40NTA4NCAxNi40NTI0TDcuMzI0MTEgMTcuMTkxMUw3LjIxMTAxIDE4LjQ2MThMNy4wODczNiAyMC45NjIyVjIxLjI0MjdINy4wNzc2NEM3LjA2MjY5IDIxLjUxNzcgNi45NzkyMSAyMS44MDM4IDYuNzk0NjkgMjIuMDYxNkM2LjU5Njc5IDIyLjMzODEgNi4zMzk1MiAyMi41MDI2IDYuMTE1OTIgMjIuNjAwN0M1LjcwOTE5IDIyLjc3OTEgNS4yNDI5NyAyMi44MDk4IDQuODkxNzUgMjIuODE2OUM0LjU0Mzg4IDIyLjgyMzkgNC4xNDYwMSAyMi44MDY1IDMuNzUyODcgMjIuNzg4MkM2LjE0MTk0IDI3LjA4OTMgMTAuNzMxIDI5Ljk5OTkgMTUuOTk5OSAyOS45OTk5QzIxLjMxMDIgMjkuOTk5OSAyNS45Mjk5IDI3LjA0MzQgMjguMzAyOCAyMi42ODY2Wk0xNS45ODUzIDguODEyNjRDMTUuODE3IDguODEyNjQgMTUuNjgwNSA4Ljk0OTA5IDE1LjY4MDUgOS4xMTc0MUMxNS42ODA1IDkuMjg1NzMgMTUuODE3IDkuNDIyMTggMTUuOTg1MyA5LjQyMjE4QzE2LjE1MzYgOS40MjIxOCAxNi4yOTAxIDkuMjg1NzMgMTYuMjkwMSA5LjExNzQxQzE2LjI5MDEgOC45NDkwOSAxNi4xNTM2IDguODEyNjQgMTUuOTg1MyA4LjgxMjY0Wk0yMi45NDkxIDEwLjgwOTZDMjIuODA4IDEwLjgwOTYgMjIuNjkzNiAxMC45MjQgMjIuNjkzNiAxMS4wNjVDMjIuNjkzNiAxMS4yMDYxIDIyLjgwOCAxMS4zMjA1IDIyLjk0OTEgMTEuMzIwNUMyMy4wOTAxIDExLjMyMDUgMjMuMjA0NSAxMS4yMDYxIDIzLjIwNDUgMTEuMDY1QzIzLjIwNDUgMTAuOTI0IDIzLjA5MDEgMTAuODA5NiAyMi45NDkxIDEwLjgwOTZaTTI3LjM0NzEgMjEuMDIzN0gyNy4zNDQ4TDI3LjMzNjkgMjAuOTA4MkMyNy4zNDM5IDIwLjk0ODYgMjcuMzQ3MSAyMC45ODc1IDI3LjM0NzEgMjEuMDIzN1pNNC44NDEwMiAyMS4xNTA3VjIxLjA2MTNDNC44MzU4NyAyMS4wOTQ0IDQuODMzNzkgMjEuMTI0NyA0LjgzMzc5IDIxLjE1MDdINC44NDEwMlpNOC43NjU1NiAxMS4xNDkxQzguNzY1NTYgMTEuMDA4IDguODc5OTMgMTAuODkzNyA5LjAyMSAxMC44OTM3QzkuMTYyMDggMTAuODkzNyA5LjI3NjQ0IDExLjAwOCA5LjI3NjQ0IDExLjE0OTFDOS4yNzY0NCAxMS4yOTAyIDkuMTYyMDggMTEuNDA0NiA5LjAyMSAxMS40MDQ2QzguODc5OTMgMTEuNDA0NiA4Ljc2NTU2IDExLjI5MDIgOC43NjU1NiAxMS4xNDkxWiIgZmlsbD0idXJsKCNwYWludDBfbGluZWFyXzFfMTE2KSIvPgo8cGF0aCBkPSJNMzguNDIzMyA5LjkwOTA5TDQxLjEwOCAxOS4xMzkySDQxLjIxMDJMNDMuODk0OSA5LjkwOTA5SDQ3LjkwOTFMNDMuNTg4MSAyM0gzOC43MzAxTDM0LjQwOTEgOS45MDkwOUgzOC40MjMzWk01Mi43MDMxIDIzLjE3OUM1MS42Mzc4IDIzLjE3OSA1MC43MjU5IDIyLjk2OCA0OS45NjczIDIyLjU0NjJDNDkuMjA4OCAyMi4xMiA0OC42MjcxIDIxLjUyNzcgNDguMjIyMyAyMC43NjkyQzQ3LjgxNzUgMjAuMDA2NCA0Ny42MTUxIDE5LjEyMjIgNDcuNjE1MSAxOC4xMTY1QzQ3LjYxNTEgMTcuMTEwOCA0Ny44MTc1IDE2LjIyODcgNDguMjIyMyAxNS40NzAyQzQ4LjYyNzEgMTQuNzA3NCA0OS4yMDg4IDE0LjExNTEgNDkuOTY3MyAxMy42OTMyQzUwLjcyNTkgMTMuMjY3IDUxLjYzNzggMTMuMDU0IDUyLjcwMzEgMTMuMDU0QzUzLjc2ODUgMTMuMDU0IDU0LjY4MDQgMTMuMjY3IDU1LjQzODkgMTMuNjkzMkM1Ni4xOTc0IDE0LjExNTEgNTYuNzc5MSAxNC43MDc0IDU3LjE4MzkgMTUuNDcwMkM1Ny41ODg4IDE2LjIyODcgNTcuNzkxMiAxNy4xMTA4IDU3Ljc5MTIgMTguMTE2NUM1Ny43OTEyIDE5LjEyMjIgNTcuNTg4OCAyMC4wMDY0IDU3LjE4MzkgMjAuNzY5MkM1Ni43NzkxIDIxLjUyNzcgNTYuMTk3NCAyMi4xMiA1NS40Mzg5IDIyLjU0NjJDNTQuNjgwNCAyMi45NjggNTMuNzY4NSAyMy4xNzkgNTIuNzAzMSAyMy4xNzlaTTUyLjcyODcgMjAuNTcxQzUzLjAyNyAyMC41NzEgNTMuMjg0OCAyMC40NzA5IDUzLjUwMjEgMjAuMjcwNkM1My43MTk1IDIwLjA3MDMgNTMuODg3OCAxOS43ODQ4IDU0LjAwNzEgMTkuNDE0MUM1NC4xMjY0IDE5LjA0MzMgNTQuMTg2MSAxOC42MDIzIDU0LjE4NjEgMTguMDkwOUM1NC4xODYxIDE3LjU3NTMgNTQuMTI2NCAxNy4xMzQyIDU0LjAwNzEgMTYuNzY3OEM1My44ODc4IDE2LjM5NyA1My43MTk1IDE2LjExMTUgNTMuNTAyMSAxNS45MTEyQzUzLjI4NDggMTUuNzEwOSA1My4wMjcgMTUuNjEwOCA1Mi43Mjg3IDE1LjYxMDhDNTIuNDEzNCAxNS42MTA4IDUyLjE0MjggMTUuNzEwOSA1MS45MTY5IDE1LjkxMTJDNTEuNjkxMSAxNi4xMTE1IDUxLjUxODUgMTYuMzk3IDUxLjM5OTEgMTYuNzY3OEM1MS4yNzk4IDE3LjEzNDIgNTEuMjIwMiAxNy41NzUzIDUxLjIyMDIgMTguMDkwOUM1MS4yMjAyIDE4LjYwMjMgNTEuMjc5OCAxOS4wNDMzIDUxLjM5OTEgMTkuNDE0MUM1MS41MTg1IDE5Ljc4NDggNTEuNjkxMSAyMC4wNzAzIDUxLjkxNjkgMjAuMjcwNkM1Mi4xNDI4IDIwLjQ3MDkgNTIuNDEzNCAyMC41NzEgNTIuNzI4NyAyMC41NzFaTTYzLjk1MzEgMjMuMTc5QzYyLjg4NzggMjMuMTc5IDYxLjk3NTkgMjIuOTY4IDYxLjIxNzMgMjIuNTQ2MkM2MC40NTg4IDIyLjEyIDU5Ljg3NzEgMjEuNTI3NyA1OS40NzIzIDIwLjc2OTJDNTkuMDY3NSAyMC4wMDY0IDU4Ljg2NTEgMTkuMTIyMiA1OC44NjUxIDE4LjExNjVDNTguODY1MSAxNy4xMTA4IDU5LjA2NzUgMTYuMjI4NyA1OS40NzIzIDE1LjQ3MDJDNTkuODc3MSAxNC43MDc0IDYwLjQ1ODggMTQuMTE1MSA2MS4yMTczIDEzLjY5MzJDNjEuOTc1OSAxMy4yNjcgNjIuODg3OCAxMy4wNTQgNjMuOTUzMSAxMy4wNTRDNjQuOTE2MiAxMy4wNTQgNjUuNzQ5MyAxMy4yMjg3IDY2LjQ1MjQgMTMuNTc4MUM2Ny4xNTk4IDEzLjkyMzMgNjcuNzA3NCAxNC40MTM0IDY4LjA5NTIgMTUuMDQ4M0M2OC40ODMgMTUuNjc5IDY4LjY3OSAxNi40MjA1IDY4LjY4MzIgMTcuMjcyN0g2NS40MTA1QzY1LjM2MzYgMTYuNzU3MSA2NS4yMTQ1IDE2LjM2NTEgNjQuOTYzMSAxNi4wOTY2QzY0LjcxNTkgMTUuODIzOSA2NC4zOTYzIDE1LjY4NzUgNjQuMDA0MyAxNS42ODc1QzYzLjY5NzQgMTUuNjg3NSA2My40MjkgMTUuNzc3IDYzLjE5ODkgMTUuOTU2QzYyLjk2ODggMTYuMTMwNyA2Mi43ODk4IDE2LjM5NyA2Mi42NjE5IDE2Ljc1NUM2Mi41MzQxIDE3LjEwODcgNjIuNDcwMiAxNy41NTQgNjIuNDcwMiAxOC4wOTA5QzYyLjQ3MDIgMTguNjI3OCA2Mi41MzQxIDE5LjA3NTMgNjIuNjYxOSAxOS40MzMyQzYyLjc4OTggMTkuNzg2OSA2Mi45Njg4IDIwLjA1MzMgNjMuMTk4OSAyMC4yMzIyQzYzLjQyOSAyMC40MDcgNjMuNjk3NCAyMC40OTQzIDY0LjAwNDMgMjAuNDk0M0M2NC4yNjQyIDIwLjQ5NDMgNjQuNDk0MyAyMC40MzQ3IDY0LjY5NDYgMjAuMzE1M0M2NC44OTQ5IDIwLjE5MTggNjUuMDU2OCAyMC4wMTI4IDY1LjE4MDQgMTkuNzc4NEM2NS4zMDgyIDE5LjUzOTggNjUuMzg0OSAxOS4yNSA2NS40MTA1IDE4LjkwOTFINjguNjgzMkM2OC42NzA1IDE5Ljc3NDEgNjguNDcyMyAyMC41Mjg0IDY4LjA4ODggMjEuMTcxOUM2Ny43MDUzIDIxLjgxMTEgNjcuMTYxOSAyMi4zMDU0IDY2LjQ1ODggMjIuNjU0OEM2NS43NTk5IDIzLjAwNDMgNjQuOTI0NyAyMy4xNzkgNjMuOTUzMSAyMy4xNzlaTTc0Ljg0MzYgMjMuMTc5QzczLjc5NTMgMjMuMTc5IDcyLjg5MTkgMjIuOTc4NyA3Mi4xMzMzIDIyLjU3ODFDNzEuMzc5MSAyMi4xNzMzIDcwLjc5NzQgMjEuNTkzNyA3MC4zODgzIDIwLjgzOTVDNjkuOTgzNSAyMC4wODEgNjkuNzgxMSAxOS4xNzMzIDY5Ljc4MTEgMTguMTE2NUM2OS43ODExIDE3LjEwMjMgNjkuOTg1NiAxNi4yMTU5IDcwLjM5NDcgMTUuNDU3NEM3MC44MDM4IDE0LjY5ODkgNzEuMzgxMiAxNC4xMDg3IDcyLjEyNyAxMy42ODY4QzcyLjg3MjcgMTMuMjY0OSA3My43NTI3IDEzLjA1NCA3NC43NjY5IDEzLjA1NEM3NS41MDgzIDEzLjA1NCA3Ni4xNzk1IDEzLjE2OSA3Ni43ODA0IDEzLjM5OTFDNzcuMzgxMiAxMy42MjkzIDc3Ljg5NDcgMTMuOTYzOCA3OC4zMjA4IDE0LjQwMjdDNzguNzQ3IDE0LjgzNzQgNzkuMDc1MSAxNS4zNjU4IDc5LjMwNTIgMTUuOTg3OUM3OS41MzUzIDE2LjYxMDEgNzkuNjUwNCAxNy4zMTExIDc5LjY1MDQgMTguMDkwOVYxOC45MDkxSDcwLjg4MDVWMTYuOTY1OUg3Ni40MDMyQzc2LjM5OSAxNi42ODQ3IDc2LjMyNjUgMTYuNDM3NSA3Ni4xODU5IDE2LjIyNDRDNzYuMDQ5NSAxNi4wMDcxIDc1Ljg2NDIgMTUuODM4OCA3NS42Mjk4IDE1LjcxOTVDNzUuMzk5NyAxNS41OTU5IDc1LjEzNzYgMTUuNTM0MSA3NC44NDM2IDE1LjUzNDFDNzQuNTU4MSAxNS41MzQxIDc0LjI5NiAxNS41OTU5IDc0LjA1NzQgMTUuNzE5NUM3My44MTg3IDE1LjgzODggNzMuNjI3IDE2LjAwNSA3My40ODIxIDE2LjIxOEM3My4zNDE0IDE2LjQzMTEgNzMuMjY2OSAxNi42ODA0IDczLjI1ODMgMTYuOTY1OVYxOS4wNjI1QzczLjI1ODMgMTkuMzc3OCA3My4zMjQ0IDE5LjY1OTEgNzMuNDU2NSAxOS45MDYyQzczLjU4ODYgMjAuMTUzNCA3My43NzgyIDIwLjM0NzMgNzQuMDI1NCAyMC40ODc5Qzc0LjI3MjUgMjAuNjI4NiA3NC41NzA4IDIwLjY5ODkgNzQuOTIwMyAyMC42OTg5Qzc1LjE2MzIgMjAuNjk4OSA3NS4zODQ4IDIwLjY2NDggNzUuNTg1MSAyMC41OTY2Qzc1Ljc4OTYgMjAuNTI4NCA3NS45NjQzIDIwLjQzMDQgNzYuMTA5MiAyMC4zMDI2Qzc2LjI1NDEgMjAuMTcwNSA3Ni4zNjA2IDIwLjAxMjggNzYuNDI4OCAxOS44Mjk1SDc5LjY1MDRDNzkuNTM5NiAyMC41MTE0IDc5LjI3NzUgMjEuMTAzNyA3OC44NjQyIDIxLjYwNjVDNzguNDUwOCAyMi4xMDUxIDc3LjkwMzIgMjIuNDkyOSA3Ny4yMjE0IDIyLjc2OTlDNzYuNTQzOSAyMy4wNDI2IDc1Ljc1MTIgMjMuMTc5IDc0Ljg0MzYgMjMuMTc5WiIgZmlsbD0iIzQzQkRDRSIvPgo8cGF0aCBkPSJNOTAuODQ2NiAxMy4xODE4QzkwLjc3NDEgMTIuNDUzMSA5MC40NTI0IDExLjg3MzYgODkuODgxNCAxMS40NDMyQzg5LjMxMDQgMTEuMDEyOCA4OC41OTIzIDEwLjc5NzYgODcuNzI3MyAxMC43OTc2Qzg3LjExNzkgMTAuNzk3NiA4Ni41ODEgMTAuOTAyIDg2LjExNjUgMTEuMTEwOEM4NS42NTYzIDExLjMxOTYgODUuMjk0IDExLjYwOTQgODUuMDI5OCAxMS45ODAxQzg0Ljc2OTkgMTIuMzQ2NiA4NC42Mzk5IDEyLjc2NDIgODQuNjM5OSAxMy4yMzNDODQuNjM5OSAxMy41NzgxIDg0LjcxNDUgMTMuODgwNyA4NC44NjM2IDE0LjE0MDZDODUuMDEyOCAxNC40MDA2IDg1LjIxMzEgMTQuNjI0MyA4NS40NjQ1IDE0LjgxMThDODUuNzIwMiAxNC45OTUgODYuMDAxNCAxNS4xNTI3IDg2LjMwODIgMTUuMjg0OEM4Ni42MTkzIDE1LjQxNjkgODYuOTMyNSAxNS41Mjc3IDg3LjI0NzkgMTUuNjE3Mkw4OC42Mjg2IDE2LjAxMzVDODkuMDQ2MiAxNi4xMjg2IDg5LjQ2MzggMTYuMjc1NiA4OS44ODE0IDE2LjQ1NDVDOTAuMjk5IDE2LjYzMzUgOTAuNjgwNCAxNi44NTk0IDkxLjAyNTYgMTcuMTMyMUM5MS4zNzUgMTcuNDAwNiA5MS42NTQxIDE3LjczMDggOTEuODYyOSAxOC4xMjI5QzkyLjA3NiAxOC41MTA3IDkyLjE4MjUgMTguOTc3MyA5Mi4xODI1IDE5LjUyMjdDOTIuMTgyNSAyMC4yMjE2IDkyLjAwMTQgMjAuODUwMSA5MS42MzkyIDIxLjQwODRDOTEuMjc3IDIxLjk2NjYgOTAuNzU3MSAyMi40MDk4IDkwLjA3OTUgMjIuNzM3OUM4OS40MDIgMjMuMDYxOCA4OC41OTAyIDIzLjIyMzcgODcuNjQ0MiAyMy4yMjM3Qzg2Ljc1MzYgMjMuMjIzNyA4NS45ODAxIDIzLjA3NjcgODUuMzIzOSAyMi43ODI3Qzg0LjY2NzYgMjIuNDg0NCA4NC4xNTIgMjIuMDczMiA4My43NzcgMjEuNTQ5QzgzLjQwMiAyMS4wMjQ5IDgzLjE5MTEgMjAuNDE3NiA4My4xNDQyIDE5LjcyNzNIODQuMzcxNEM4NC40MTQxIDIwLjI0MjkgODQuNTg0NSAyMC42Nzk3IDg0Ljg4MjggMjEuMDM3NkM4NS4xODExIDIxLjM5NTYgODUuNTY4OSAyMS42NjgzIDg2LjA0NjIgMjEuODU1OEM4Ni41MjM0IDIyLjAzOTEgODcuMDU2MSAyMi4xMzA3IDg3LjY0NDIgMjIuMTMwN0M4OC4yOTYyIDIyLjEzMDcgODguODc1NyAyMi4wMjIgODkuMzgyOCAyMS44MDQ3Qzg5Ljg5NDIgMjEuNTgzMSA5MC4yOTQ3IDIxLjI3NjMgOTAuNTg0NSAyMC44ODQyQzkwLjg3ODYgMjAuNDg3OSA5MS4wMjU2IDIwLjAyNzcgOTEuMDI1NiAxOS41MDM2QzkxLjAyNTYgMTkuMDYwNCA5MC45MTA1IDE4LjY4OTYgOTAuNjgwNCAxOC4zOTEzQzkwLjQ1MDMgMTguMDg4OCA5MC4xMjg2IDE3LjgzNTIgODkuNzE1MiAxNy42MzA3Qzg5LjMwNjEgMTcuNDI2MSA4OC44MzEgMTcuMjQ1IDg4LjI4OTggMTcuMDg3NEw4Ni43MjM3IDE2LjYyNzFDODUuNjk2NyAxNi4zMjAzIDg0Ljg5OTkgMTUuODk2MyA4NC4zMzMxIDE1LjM1NTFDODMuNzY2MyAxNC44MTM5IDgzLjQ4MyAxNC4xMjM2IDgzLjQ4MyAxMy4yODQxQzgzLjQ4MyAxMi41ODEgODMuNjcwNSAxMS45NjMxIDg0LjA0NTUgMTEuNDMwNEM4NC40MjQ3IDEwLjg5MzUgODQuOTM2MSAxMC40NzU5IDg1LjU3OTUgMTAuMTc3NkM4Ni4yMjczIDkuODc1IDg2Ljk1MzggOS43MjM3MiA4Ny43NTkyIDkuNzIzNzJDODguNTczMiA5LjcyMzcyIDg5LjI5MzMgOS44NzI4NyA4OS45MTk3IDEwLjE3MTJDOTAuNTQ2MiAxMC40Njk1IDkxLjA0MjYgMTAuODgwNyA5MS40MDkxIDExLjQwNDhDOTEuNzc5OCAxMS45MjQ3IDkxLjk3OCAxMi41MTcgOTIuMDAzNiAxMy4xODE4SDkwLjg0NjZaTTk0LjgyMjQgMjYuNjgxOFYxMy4xODE4SDk1LjkyODNWMTUuMDU0N0g5Ni4wNjI1Qzk2LjE5ODkgMTQuNzYwNyA5Ni4zODY0IDE0LjQ2MDIgOTYuNjI1IDE0LjE1MzRDOTYuODYzNiAxMy44NDIzIDk3LjE3OSAxMy41ODAzIDk3LjU3MSAxMy4zNjcyQzk3Ljk2NzMgMTMuMTU0MSA5OC40NjU5IDEzLjA0NzYgOTkuMDY2OCAxMy4wNDc2Qzk5Ljg3NjQgMTMuMDQ3NiAxMDAuNTg0IDEzLjI2MDcgMTAxLjE4OSAxMy42ODY4QzEwMS43OTggMTQuMTA4NyAxMDIuMjcxIDE0LjcwMSAxMDIuNjA4IDE1LjQ2MzhDMTAyLjk0OSAxNi4yMjIzIDEwMy4xMTkgMTcuMTA2NSAxMDMuMTE5IDE4LjExNjVDMTAzLjExOSAxOS4xMzA3IDEwMi45NDkgMjAuMDE5MiAxMDIuNjA4IDIwLjc4MkMxMDIuMjcxIDIxLjU0NDcgMTAxLjc5OCAyMi4xMzkyIDEwMS4xODkgMjIuNTY1M0MxMDAuNTg0IDIyLjk5MTUgOTkuODgwNyAyMy4yMDQ1IDk5LjA3OTUgMjMuMjA0NUM5OC40ODcyIDIzLjIwNDUgOTcuOTkwOCAyMy4wOTggOTcuNTkwMiAyMi44ODQ5Qzk3LjE5MzkgMjIuNjcxOSA5Ni44NzIyIDIyLjQwOTggOTYuNjI1IDIyLjA5ODdDOTYuMzgyMSAyMS43ODM0IDk2LjE5NDYgMjEuNDc2NiA5Ni4wNjI1IDIxLjE3ODNIOTUuOTYwMlYyNi42ODE4SDk0LjgyMjRaTTk1Ljk0NzQgMTguMDk3M0M5NS45NDc0IDE4Ljg4OTkgOTYuMDY0NiAxOS41OTMgOTYuMjk5IDIwLjIwNjdDOTYuNTM3NiAyMC44MTYxIDk2Ljg3ODYgMjEuMjk1NSA5Ny4zMjE3IDIxLjY0NDlDOTcuNzY5MiAyMS45OTAxIDk4LjMxMDQgMjIuMTYyNiA5OC45NDUzIDIyLjE2MjZDOTkuNTk3MyAyMi4xNjI2IDEwMC4xNDcgMjEuOTgzNyAxMDAuNTk0IDIxLjYyNTdDMTAxLjA0NiAyMS4yNjM1IDEwMS4zODkgMjAuNzc1NiAxMDEuNjI0IDIwLjE2MTlDMTAxLjg2MiAxOS41NDgzIDEwMS45ODIgMTguODYwMSAxMDEuOTgyIDE4LjA5NzNDMTAxLjk4MiAxNy4zNDMgMTAxLjg2NCAxNi42NjM0IDEwMS42MyAxNi4wNTgyQzEwMS40IDE1LjQ1MzEgMTAxLjA1OSAxNC45NzM3IDEwMC42MDcgMTQuNjJDMTAwLjE1NiAxNC4yNjIxIDk5LjYwMTYgMTQuMDgzMSA5OC45NDUzIDE0LjA4MzFDOTguMzA2MSAxNC4wODMxIDk3Ljc2MjggMTQuMjU1NyA5Ny4zMTUzIDE0LjYwMDlDOTYuODY3OSAxNC45NDE4IDk2LjUyNyAxNS40MTQ4IDk2LjI5MjYgMTYuMDE5OUM5Ni4wNjI1IDE2LjYyMDcgOTUuOTQ3NCAxNy4zMTMyIDk1Ljk0NzQgMTguMDk3M1pNMTA4LjM1OCAyMy4yMjM3QzEwNy43NjUgMjMuMjIzNyAxMDcuMjI0IDIzLjEwODcgMTA2LjczNCAyMi44Nzg2QzEwNi4yNDQgMjIuNjQ0MiAxMDUuODU0IDIyLjMwNzUgMTA1LjU2NCAyMS44Njg2QzEwNS4yNzUgMjEuNDI1NCAxMDUuMTMgMjAuODg4NSAxMDUuMTMgMjAuMjU3OEMxMDUuMTMgMTkuNzcyIDEwNS4yMjEgMTkuMzYyOSAxMDUuNDA0IDE5LjAzMDVDMTA1LjU4OCAxOC42OTgyIDEwNS44NDggMTguNDI1NCAxMDYuMTg0IDE4LjIxMjRDMTA2LjUyMSAxNy45OTkzIDEwNi45MTkgMTcuODMxIDEwNy4zOCAxNy43MDc0QzEwNy44NCAxNy41ODM4IDEwOC4zNDcgMTcuNDg3OSAxMDguOTAxIDE3LjQxOTdDMTA5LjQ1MSAxNy4zNTE2IDEwOS45MTUgMTcuMjkxOSAxMTAuMjk0IDE3LjI0MDhDMTEwLjY3OCAxNy4xODk2IDExMC45NyAxNy4xMDg3IDExMS4xNyAxNi45OTc5QzExMS4zNyAxNi44ODcxIDExMS40NzEgMTYuNzA4MSAxMTEuNDcxIDE2LjQ2MDlWMTYuMjMwOEMxMTEuNDcxIDE1LjU2MTggMTExLjI3IDE1LjAzNTUgMTEwLjg3IDE0LjY1MkMxMTAuNDczIDE0LjI2NDIgMTA5LjkwMiAxNC4wNzAzIDEwOS4xNTcgMTQuMDcwM0MxMDguNDQ5IDE0LjA3MDMgMTA3Ljg3MiAxNC4yMjU5IDEwNy40MjQgMTQuNTM2OUMxMDYuOTgxIDE0Ljg0OCAxMDYuNjcgMTUuMjE0NSAxMDYuNDkxIDE1LjYzNjRMMTA1LjQxMSAxNS4yNDY0QzEwNS42MzIgMTQuNzA5NSAxMDUuOTM5IDE0LjI4MTIgMTA2LjMzMSAxMy45NjE2QzEwNi43MjMgMTMuNjM3OCAxMDcuMTYyIDEzLjQwNTUgMTA3LjY0OCAxMy4yNjQ5QzEwOC4xMzQgMTMuMTIgMTA4LjYyNiAxMy4wNDc2IDEwOS4xMjUgMTMuMDQ3NkMxMDkuNSAxMy4wNDc2IDEwOS44OSAxMy4wOTY2IDExMC4yOTQgMTMuMTk0NkMxMTAuNzAzIDEzLjI5MjYgMTExLjA4MyAxMy40NjMxIDExMS40MzIgMTMuNzA2QzExMS43ODIgMTMuOTQ0NiAxMTIuMDY1IDE0LjI3OTEgMTEyLjI4MiAxNC43MDk1QzExMi41IDE1LjEzNTcgMTEyLjYwOCAxNS42NzY4IDExMi42MDggMTYuMzMzMVYyM0gxMTEuNDcxVjIxLjQ0NjdIMTExLjRDMTExLjI2NCAyMS43MzY1IDExMS4wNjEgMjIuMDE3OCAxMTAuNzkzIDIyLjI5MDVDMTEwLjUyNSAyMi41NjMyIDExMC4xODggMjIuNzg2OSAxMDkuNzgzIDIyLjk2MTZDMTA5LjM3OCAyMy4xMzY0IDEwOC45MDMgMjMuMjIzNyAxMDguMzU4IDIzLjIyMzdaTTEwOC41MTEgMjIuMTgxOEMxMDkuMTE2IDIyLjE4MTggMTA5LjY0IDIyLjA0NzYgMTEwLjA4MyAyMS43NzkxQzExMC41MjcgMjEuNTEwNyAxMTAuODY4IDIxLjE1NDggMTExLjEwNiAyMC43MTE2QzExMS4zNDkgMjAuMjY0MiAxMTEuNDcxIDE5Ljc3MiAxMTEuNDcxIDE5LjIzNTFWMTcuODE2MUMxMTEuMzg1IDE3Ljg5NyAxMTEuMjQzIDE3Ljk2OTUgMTExLjA0MiAxOC4wMzM0QzExMC44NDYgMTguMDk3MyAxMTAuNjE4IDE4LjE1NDggMTEwLjM1OCAxOC4yMDZDMTEwLjEwMyAxOC4yNTI4IDEwOS44NDcgMTguMjkzMyAxMDkuNTkxIDE4LjMyNzRDMTA5LjMzNiAxOC4zNjE1IDEwOS4xMDUgMTguMzkxMyAxMDguOTAxIDE4LjQxNjlDMTA4LjM0NyAxOC40ODUxIDEwNy44NzQgMTguNTkxNiAxMDcuNDgyIDE4LjczNjVDMTA3LjA5IDE4Ljg4MTQgMTA2Ljc4OSAxOS4wODE3IDEwNi41ODEgMTkuMzM3NEMxMDYuMzcyIDE5LjU4ODggMTA2LjI2NyAxOS45MTI2IDEwNi4yNjcgMjAuMzA4OUMxMDYuMjY3IDIwLjkwNTUgMTA2LjQ4IDIxLjM2NzkgMTA2LjkwNyAyMS42OTZDMTA3LjMzMyAyMi4wMTk5IDEwNy44NjggMjIuMTgxOCAxMDguNTExIDIyLjE4MThaTTExOS40NyAyMy4yMDQ1QzExOC41OTIgMjMuMjA0NSAxMTcuODI3IDIyLjk4NTEgMTE3LjE3NSAyMi41NDYyQzExNi41MjggMjIuMTA3MiAxMTYuMDI1IDIxLjUwNjQgMTE1LjY2NyAyMC43NDM2QzExNS4zMDkgMTkuOTgwOCAxMTUuMTMgMTkuMTExNSAxMTUuMTMgMTguMTM1N0MxMTUuMTMgMTcuMTUxMyAxMTUuMzExIDE2LjI3NTYgMTE1LjY3MyAxNS41MDg1QzExNi4wNCAxNC43NDE1IDExNi41NDcgMTQuMTQwNiAxMTcuMTk1IDEzLjcwNkMxMTcuODQyIDEzLjI2NyAxMTguNTk0IDEzLjA0NzYgMTE5LjQ1MSAxMy4wNDc2QzEyMC4xMDcgMTMuMDQ3NiAxMjAuNzAyIDEzLjE3NTQgMTIxLjIzNCAxMy40MzExQzEyMS43NjcgMTMuNjgyNSAxMjIuMjA2IDE0LjAzODQgMTIyLjU1MSAxNC40OTg2QzEyMi45MDEgMTQuOTU0NSAxMjMuMTIgMTUuNDg3MiAxMjMuMjEgMTYuMDk2NkgxMjIuMDU5QzEyMS45NCAxNS41NDI2IDEyMS42NTIgMTUuMDY5NiAxMjEuMTk2IDE0LjY3NzZDMTIwLjc0NCAxNC4yODEyIDEyMC4xNjkgMTQuMDgzMSAxMTkuNDcgMTQuMDgzMUMxMTguODQ0IDE0LjA4MzEgMTE4LjI5IDE0LjI1MzYgMTE3LjgwOCAxNC41OTQ1QzExNy4zMjcgMTQuOTMxMSAxMTYuOTUgMTUuNDAyIDExNi42NzcgMTYuMDA3MUMxMTYuNDA4IDE2LjYwOCAxMTYuMjc0IDE3LjMwNDcgMTE2LjI3NCAxOC4wOTczQzExNi4yNzQgMTguODk0MiAxMTYuNDA2IDE5LjU5OTQgMTE2LjY3IDIwLjIxMzFDMTE2LjkzNSAyMC44MjI0IDExNy4zMDUgMjEuMjk5NyAxMTcuNzgzIDIxLjY0NDlDMTE4LjI2NCAyMS45OTAxIDExOC44MjcgMjIuMTYyNiAxMTkuNDcgMjIuMTYyNkMxMTkuOTA1IDIyLjE2MjYgMTIwLjMwMSAyMi4wODE3IDEyMC42NTkgMjEuOTE5N0MxMjEuMDIxIDIxLjc1MzYgMTIxLjMyNCAyMS41MTkyIDEyMS41NjcgMjEuMjE2NkMxMjEuODE0IDIwLjkxNDEgMTIxLjk4IDIwLjU1NCAxMjIuMDY1IDIwLjEzNjRIMTIzLjIxNkMxMjMuMTMxIDIwLjcyODcgMTIyLjkyIDIxLjI1NzEgMTIyLjU4MyAyMS43MjE2QzEyMi4yNTEgMjIuMTgxOCAxMjEuODE4IDIyLjU0NCAxMjEuMjg2IDIyLjgwODJDMTIwLjc1NyAyMy4wNzI0IDEyMC4xNTIgMjMuMjA0NSAxMTkuNDcgMjMuMjA0NVpNMTI5LjYyNCAyMy4yMDQ1QzEyOC43MDggMjMuMjA0NSAxMjcuOTE1IDIyLjk5MTUgMTI3LjI0NiAyMi41NjUzQzEyNi41NzcgMjIuMTM0OSAxMjYuMDU5IDIxLjU0MDUgMTI1LjY5MyAyMC43ODJDMTI1LjMzMSAyMC4wMTkyIDEyNS4xNSAxOS4xNDEzIDEyNS4xNSAxOC4xNDg0QzEyNS4xNSAxNy4xNTk4IDEyNS4zMzEgMTYuMjgyIDEyNS42OTMgMTUuNTE0OUMxMjYuMDU5IDE0Ljc0MzYgMTI2LjU2NCAxNC4xNDA2IDEyNy4yMDggMTMuNzA2QzEyNy44NTUgMTMuMjY3IDEyOC42MDMgMTMuMDQ3NiAxMjkuNDUxIDEzLjA0NzZDMTI5Ljk4NCAxMy4wNDc2IDEzMC40OTggMTMuMTQ1NiAxMzAuOTkyIDEzLjM0MTZDMTMxLjQ4NiAxMy41MzM0IDEzMS45MjkgMTMuODI5NSAxMzIuMzIxIDE0LjIzMDFDMTMyLjcxOCAxNC42MjY0IDEzMy4wMzEgMTUuMTI3MSAxMzMuMjYxIDE1LjczMjJDMTMzLjQ5MSAxNi4zMzMxIDEzMy42MDYgMTcuMDQ0NyAxMzMuNjA2IDE3Ljg2NzJWMTguNDI5N0gxMjUuOTM2VjE3LjQyNjFIMTMyLjQ0M0MxMzIuNDQzIDE2Ljc5NTUgMTMyLjMxNSAxNi4yMjg3IDEzMi4wNTkgMTUuNzI1OUMxMzEuODA4IDE1LjIxODcgMTMxLjQ1NiAxNC44MTgyIDEzMS4wMDUgMTQuNTI0MUMxMzAuNTU3IDE0LjIzMDEgMTMwLjAzOSAxNC4wODMxIDEyOS40NTEgMTQuMDgzMUMxMjguODI5IDE0LjA4MzEgMTI4LjI4MiAxNC4yNDkzIDEyNy44MDkgMTQuNTgxN0MxMjcuMzM2IDE0LjkxNDEgMTI2Ljk2NSAxNS4zNTMgMTI2LjY5NiAxNS44OTg0QzEyNi40MzIgMTYuNDQzOSAxMjYuMjk4IDE3LjA0MDUgMTI2LjI5NCAxNy42ODgyVjE4LjI4OTFDMTI2LjI5NCAxOS4wNjg5IDEyNi40MjggMTkuNzUwNyAxMjYuNjk2IDIwLjMzNDVDMTI2Ljk2OSAyMC45MTQxIDEyNy4zNTUgMjEuMzYzNiAxMjcuODUzIDIxLjY4MzJDMTI4LjM1MiAyMi4wMDI4IDEyOC45NDIgMjIuMTYyNiAxMjkuNjI0IDIyLjE2MjZDMTMwLjA4OCAyMi4xNjI2IDEzMC40OTUgMjIuMDkwMiAxMzAuODQ1IDIxLjk0NTNDMTMxLjE5OSAyMS44MDA0IDEzMS40OTUgMjEuNjA2NSAxMzEuNzMzIDIxLjM2MzZDMTMxLjk3NiAyMS4xMTY1IDEzMi4xNTkgMjAuODQ1OSAxMzIuMjgzIDIwLjU1MThMMTMzLjM2MyAyMC45MDM0QzEzMy4yMTQgMjEuMzE2OCAxMzIuOTY5IDIxLjY5ODIgMTMyLjYyOCAyMi4wNDc2QzEzMi4yOTIgMjIuMzk3IDEzMS44NyAyMi42NzgzIDEzMS4zNjMgMjIuODkxM0MxMzAuODYgMjMuMTAwMSAxMzAuMjggMjMuMjA0NSAxMjkuNjI0IDIzLjIwNDVaIiBmaWxsPSJ3aGl0ZSIvPgo8ZGVmcz4KPGxpbmVhckdyYWRpZW50IGlkPSJwYWludDBfbGluZWFyXzFfMTE2IiB4MT0iNy42OTUyNyIgeTE9IjIyLjA1ODMiIHgyPSIyMS4xNTk3IiB5Mj0iMTEuNDM5MiIgZ3JhZGllbnRVbml0cz0idXNlclNwYWNlT25Vc2UiPgo8c3RvcCBzdG9wLWNvbG9yPSIjMDU2Q0YyIi8+CjxzdG9wIG9mZnNldD0iMSIgc3RvcC1jb2xvcj0iIzRFQ0NDNiIvPgo8L2xpbmVhckdyYWRpZW50Pgo8L2RlZnM+Cjwvc3ZnPgo="
                              alt="Hello World!"
                              class="el-tooltip__trigger"
                              style="
                                width: 220px;
                                height: 90px;
                                font-weight: 100;
                                margin: 30px 0px 9px;
                                padding: 0px;
                                background-color: rgb(33, 36, 41);
                                font-family: Helvetica;
                                border-radius: 0px;
                                display: flex;
                                align-items: center;
                                justify-content: center;
                                border-width: 0px;
                                border-style: solid;
                                border-color: rgb(255, 255, 255);
                              "
                            />
                          </td>
                        </tr>
                        
                        <tr data-v-57b2d1f5="" class="el-tooltip__trigger">
                          <td
                            data-v-57b2d1f5=""
                            style="
                              background-color: rgb(33, 36, 41);
                              text-align: center;
                              display: flex;
                              align-items: center;
                              justify-content: center;
                              flex-direction: column;
                            "
                          >
                            <span
                              data-v-57b2d1f5=""
                              class="el-tooltip__trigger"
                              style="
                                font-size: 16px;
                                color: rgb(111, 119, 134);
                                text-align: center;
                                display: flex;
                                line-height: 1.5em;
                                width: 100%;
                                height: 30px;
                                font-weight: 700;
                                margin: 0px 0px 16px;
                                padding: 0px;
                                background-color: rgb(33, 36, 41);
                                font-family: Helvetica;
                                border-radius: 0px;
                                align-items: center;
                                justify-content: start;
                                border-width: 0px;
                                border-style: solid;
                                border-color: rgb(255, 255, 255);
                              "
                              >Thank you for purchasing VoceSpace Professional
                              Edition. The following is your License. Please
                              keep it safe.</span
                            >
                          </td>
                        </tr>
                        <tr data-v-57b2d1f5="" class="el-tooltip__trigger">
                          <td
                            data-v-57b2d1f5=""
                            style="
                              background-color: rgb(23, 25, 28);
                              text-align: center;
                              display: flex;
                              align-items: center;
                              justify-content: center;
                              flex-direction: column;
                            "
                          >
                            <span
                              data-v-57b2d1f5=""
                              class="el-tooltip__trigger"
                              style="
                                font-size: 16px;
                                color: #22CCEE;
                                text-align: center;
                                display: flex;
                                line-height: 1.5em;
                                width: 90%;
                                height: 45px;
                                font-weight: 700;
                                margin: 10px 0px 0px;
                                padding: 0px;
                                background-color: rgb(31, 31, 32);
                                font-family: Verdana;
                                border-radius: 0px;
                                align-items: center;
                                justify-content: center;
                                border-width: 0px;
                                border-style: solid;
                                border-color: rgb(255, 255, 255);
                              "
                              >License</span
                            ><span
                              data-v-57b2d1f5=""
                              class="el-tooltip__trigger"
                              style="
                                font-size: 16px;
                                color: #fff;
                                text-align: center;
                                display: flex;
                                line-height: 1.5em;
                                width: 90%;
                                height: 60px;
                                font-weight: 500;
                                margin: 0px 0px 10px;
                                padding: 0px;
                                background-color: rgb(31, 31, 32);
                                font-family: Verdana;
                                border-radius: 0px;
                                align-items: center;
                                justify-content: center;
                                border-width: 0px;
                                border-style: solid;
                                border-color: rgb(255, 255, 255);
                              "
                              >${license}</span
                            >
                          </td>
                        </tr>
                        <tr data-v-57b2d1f5="" class="el-tooltip__trigger">
                          <td
                            data-v-57b2d1f5=""
                            style="
                              background-color: rgb(33, 36, 41);
                              text-align: center;
                              display: flex;
                              align-items: center;
                              justify-content: center;
                              flex-direction: column;
                            "
                          >
                            <span
                              data-v-57b2d1f5=""
                              class="el-tooltip__trigger"
                              style="
                                font-size: 14px;
                                color: rgb(174, 192, 209);
                                text-align: justify;
                                display: flex;
                                line-height: 1.5em;
                                width: 100%;
                                height: auto;
                                font-weight: 700;
                                margin: 16px 0px 0px;
                                padding: 0px;
                                background-color: rgb(33, 36, 41);
                                font-family: Helvetica;
                                border-radius: 0px;
                                align-items: center;
                                justify-content: center;
                                border-width: 0px;
                                border-style: solid;
                                border-color: rgb(255, 255, 255);
                              "
                              >You have received this automatically generated
                              email because you have successfully purchased
                              VoceSpace Professional Edition. This email is
                              automatically sent by the system service. Please
                              do not reply.</span
                            >
                          </td>
                        </tr>
                        <tr data-v-57b2d1f5="" class="el-tooltip__trigger">
                          <td
                            data-v-57b2d1f5=""
                            style="
                              background-color: rgb(33, 36, 41);
                              text-align: center;
                              display: flex;
                              align-items: center;
                              justify-content: center;
                              flex-direction: column;
                            "
                          >
                            <span
                              data-v-57b2d1f5=""
                              class="el-tooltip__trigger"
                              style="
                                font-size: 14px;
                                color: rgb(174, 192, 209);
                                text-align: left;
                                display: flex;
                                line-height: 1.5em;
                                width: 100%;
                                height: auto;
                                font-weight: 700;
                                margin: 30px 0px 9px;
                                padding: 0px;
                                background-color: rgb(33, 36, 41);
                                font-family: Helvetica;
                                border-radius: 0px;
                                align-items: center;
                                justify-content: start;
                                border-width: 0px;
                                border-style: solid;
                                border-color: rgb(255, 255, 255);
                              "
                              >If you have any questions or need any help,
                              please contact</span
                            ><a
                              data-v-57b2d1f5=""
                              href="han@privoce.com"
                              target="blank"
                              class="el-tooltip__trigger"
                              style="
                                font-size: 14px;
                                color: #22CCEE;
                                text-align: center;
                                display: flex;
                                line-height: 1.5em;
                                height: 30px;
                                font-weight: 700;
                                margin: 0px;
                                padding: 0px;
                                background-color: rgb(33, 36, 41);
                                font-family: Helvetica;
                                border-radius: 0px;
                                align-items: center;
                                justify-content: end;
                                border-width: 0px;
                                border-style: solid;
                                border-color: rgb(255, 255, 255);
                                width: 100%;
                              "
                              >Email: han@privoce.com</a
                            ><span
                              data-v-57b2d1f5=""
                              class="el-tooltip__trigger"
                              style="
                                font-size: 14px;
                                color: #22CCEE;
                                text-align: left;
                                display: flex;
                                line-height: 1.5em;
                                height: 30px;
                                font-weight: 700;
                                margin: 0px;
                                padding: 0px;
                                background-color: rgb(33, 36, 41);
                                font-family: Helvetica;
                                border-radius: 0px;
                                align-items: center;
                                justify-content: end;
                                border-width: 0px;
                                border-style: solid;
                                border-color: rgb(255, 255, 255);
                                width: 100%;
                              "
                              >WeChat: Privoce</span
                            >
                          </td>
                        </tr>
                      </tbody>
                    </table>
                  </td>
                </tr>
              </tbody>
            </table>
          </td>
        </tr>
      </tbody>
    </table>
  </body>
</html>
"##;

pub fn fmt_content_buy(license: &str) -> String {
    SMTP_CONTENT_PUY.replace("${license}", license)
}
