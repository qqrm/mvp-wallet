# 09. Комплаенс (Узбекистан, 2025–2026)

MVP не включает KYC и внешние платёжные рельсы, но обязателен bank-grade уровень:
- корректность леджера
- приватность данных
- аудитируемость

Ключевые темы:
- Закон о персональных данных (обязанности оператора, защита и конфиденциальность).
- AML/CFT контекст: даже без KYC требуется готовность по auditability и monitoring hooks.
- Локализация данных и трансграничная обработка — как юридическое решение; держать возможность оншорного деплоя.
- Кибербезопасность (возможная классификация как КИИ).

Требование → инженерный контроль (MVP):
- RBAC, access logs, TLS, минимизация PII, retention policy.
- Postgres backups + PITR; restore drill; migrations gated in CI.
- Runbook по расследованию расхождений баланса через tx_id receipts.

См. первоисточник: [SOURCE 1 §p028](../../../../blueprint/v3.3/appendix/sources/source_1_blueprint_v3_1.md#p028)–[SOURCE 1 §p029](../../../../blueprint/v3.3/appendix/sources/source_1_blueprint_v3_1.md#p029) и [SOURCE 1 §p032](../../../../blueprint/v3.3/appendix/sources/source_1_blueprint_v3_1.md#p032).
