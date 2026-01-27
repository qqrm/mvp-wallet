# 13. Mobile UI/UX (MVP)

Навигация:
- Bottom tabs: Home / Transfers / History / Settings (3–4 вкладки).

Принципы:
- available balance всегда консистентен с квитанциями
- каждое действие имеет review step и выдаёт receipt с tx_id
- ошибки: user-friendly текст + скрытый error.code

Список экранов (>= 12):
- Splash / Session restore
- Login (phone + OTP)
- Home
- Accounts list (если multi-currency)
- Account details (balance breakdown)
- Transaction list (filters + pagination)
- Transaction details / receipt
- Transfer form
- Transfer review + confirm
- FX convert form (quote)
- FX quote review + execute result
- Spend simulation (sandbox)
- Settings & Security
- Limits (read-only)

См. первоисточник: [SOURCE 1 §p019](../../../../blueprint/v3.3/appendix/sources/source_materials.md)–[SOURCE 1 §p025](../../../../blueprint/v3.3/appendix/sources/source_materials.md).
