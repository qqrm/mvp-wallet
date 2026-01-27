# 08. Безопасность

- Аутентификация: JWT/OAuth2 через Uzum identity; refresh tokens; (минимально) device/session binding.
- Авторизация: роли USER и ADMIN; admin endpoints требуют ADMIN; все admin действия логируются в audit.
- Шифрование: TLS in transit; at-rest через дисковые механизмы Postgres/KMS; минимизация чувствительных полей в MVP.
- Rate limits: per-user и per-IP на transfer/fx/spend/login (через gateway или middleware).

См. первоисточник: [SOURCE 1 §p009](../../../../blueprint/v3.3/appendix/sources/source_materials.md) и [SOURCE 1 §p038](../../../../blueprint/v3.3/appendix/sources/source_materials.md).
