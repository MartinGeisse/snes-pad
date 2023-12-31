
int soundState = 0;
int buttonState[16] = {0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, };

void setup() {
  Serial.begin(38400);
  pinMode(7, OUTPUT);
  pinMode(6, OUTPUT);
  pinMode(5, INPUT);
  digitalWrite(7, HIGH);
  digitalWrite(6, HIGH);

  pinMode(2, OUTPUT);
  pinMode(8, OUTPUT);
  pinMode(9, OUTPUT);
  pinMode(12, INPUT);

  for (int j = 0; j < 33; j++) {
    analogWrite(2, 255);
    delay(1);
    analogWrite(2, 0);
    delay(1);
  }
}

void loop() {
  digitalWrite(6, HIGH);
  delayMicroseconds(20);
  digitalWrite(6, LOW);
  delayMicroseconds(20);
  for (int i = 0; i < 16; i++) {
    buttonState[i] = digitalRead(5) == LOW;
    digitalWrite(7, LOW);
    delayMicroseconds(20);
    digitalWrite(7, HIGH);
    delayMicroseconds(20);
  }
  Serial.write(
    (buttonState[0] ? 1 : 0) +
    (buttonState[1] ? 2 : 0) +
    (buttonState[2] ? 4 : 0) +
    (buttonState[3] ? 8 : 0) +
    (buttonState[4] ? 16 : 0) +
    (buttonState[5] ? 32 : 0)
  );
  Serial.write(
    (buttonState[6] ? 1 : 0) +
    (buttonState[7] ? 2 : 0) +
    (buttonState[8] ? 4 : 0) +
    (buttonState[9] ? 8 : 0) +
    (buttonState[10] ? 16 : 0) +
    (buttonState[11] ? 32 : 0) +
    128
  );
}
