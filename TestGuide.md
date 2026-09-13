# Команды для тестирования API

## Подготовка

```bash
# Установи BASE (порт из твоего .env)
export BASE="http://localhost:8080"
echo "BASE: $BASE"
```

Убедись, что:
- Сервер запущен: 'cargo run'
- Миграции применены: 'sqlx migrate run'
- Установлены 'curl' и 'jq'

---

## 1. Регистрация пользователя:

```bash
# Алиса
curl -i -X POST $BASE/auth/register \
-H "Content-Type: application/json" \
-d '{
    "username": "alice",
    "email": "alice@example.com",
    "password": "SecurePass123"
}'

#Боб
curl -i -X POST $BASE/auth/register \
-H "Content-Type: application/json" \
-d '{
    "username": "bob",
    "email": "bob@example.com",
    "password": "SecurePass456"
}'
```
**Ожидаемый результат:** `200 OK` + JSON с профилем.

---

## 2. Проверка валидации:

```bash
curl -i -X POST $BASE/auth/register \
-H "Content-Type: application/json" \
-d '{
    "username": "x",
    "email": "not-an-email",
    "password": "123"
}'
```
**Ожидаемый результат:** `400 Bad Request` с сообщениями об ошибках валидации.

---

## 3. Получение JWT-токенов:

```bash
ALICE_TOKEN=$(curl -s -X POST $BASE/auth/login \
-H "Content-Type: application/json" \
-d '{"email": "alice@example.com", "password": "SecurePass123"}' | jq -r .token)

BOB_TOKEN=$(curl -s -X POST $BASE/auth/login \
-H "Content-Type: application/json" \
-d '{"email": "bob@example.com", "password": "SecurePass456"}' | jq -r .token)

echo "ALICE: $ALICE_TOKEN"
echo "BOB:   $BOB_TOKEN"
```
**Ожидаемый результат:** два JWT-токена (непустые).

---

## 4. Получение ID пользователей из БД

```bash
psql $DATABASE_URL -c "SELECT id, username FROM users;"

export ALICE_ID="UUID Алисы"
export BOB_ID="UUID Боба"

echo "ALICE_ID: $ALICE_ID"
echo "BOB_ID:   $BOB_ID"
```

---

## 5. Создание приватного чата

```bash
curl -i -X POST $BASE/chats \
-H "Authorization: Bearer $ALICE_TOKEN" \
-H "Content-Type: application/json" \
-d "{\"target_user_id\": \"$BOB_ID\"}"

export CHAT_ID="UUID чата из ответа"

echo "CHAT_ID: $CHAT_ID"
```
**Ожидаемый результат:** `201 Created` + `{"chat_id":"..."}`.

---

## 6. Проверка защиты: чат с самим собой

```bash
curl -i -X POST $BASE/chats \
-H "Authorization: Bearer $ALICE_TOKEN" \
-H "Content-Type: application/json" \
-d "{\"target_user_id\": \"$ALICE_ID\"}"
```

**Ожидаемый результат:** `400 Bad Request`.

---

## 7. Создание группового чата

```bash
curl -i -X POST $BASE/chats/group \
-H "Authorization: Bearer $ALICE_TOKEN" \
-H "Content-Type: application/json" \
-d "{
    \"title\": \"test_group\",
    \"members\": [\"$BOB_ID\"]
}"

export GROUP_CHAT_ID="UUID чата из ответа"

echo "GROUP_CHAT_ID: $GROUP_CHAT_ID"

```

**Ожидаемый результат:** `201 Created`.

---

## 8. Отправка сообщения

```bash
curl -i -X POST $BASE/chats/$CHAT_ID/messages \
-H "Authorization: Bearer $ALICE_TOKEN" \
-H "Content-Type: application/json" \
-d '{"content": "Привет, Боб! (REST)"}'
```

**Ожидаемый результат:** `201 Created` + объект `Message` с `id`, `created_at`, `deleted_at: null`.

---

## 9. Получение истории сообщений

```bash
curl -i "$BASE/chats/$CHAT_ID/messages?limit=10&offset=0" \
-H "Authorization: Bearer $ALICE_TOKEN"
```

**Ожидаемый результат:** `200 OK` + массив сообщений.

---

## 10. Мягкое удаление сообщения

```bash
export MSG_ID="UUID сообщения"

echo "MSG_ID: $MSG_ID"

curl -i -X DELETE $BASE/messages/$MSG_ID \
-H "Authorization: Bearer $ALICE_TOKEN"
```

**Ожидаемый результат:** `204 No Content`.

Проверь, что сообщение исчезло из истории:

```bash
curl -s "$BASE/chats/$CHAT_ID/messages?limit=10&offset=0" \
-H "Authorization: Bearer $ALICE_TOKEN" | jq
```

---

## 11. WebSocket

Установи `websocat`:

```bash
cargo install websocat
```

Открой два терминала:

**Терминал 1 (Bob):**

```bash
websocat "ws://localhost:8080/ws?token=$BOB_TOKEN"
```

**Терминал 2 (Alice):**

```bash
websocat "ws://localhost:8080/ws?token=$ALICE_TOKEN"
```

Отправь в терминале Alice:

```json
{"chat_id": "ID чата", "content": "Привет через WebSocket!"}
```

**Ожидаемый результат:** Bob получает JSON с сообщением.