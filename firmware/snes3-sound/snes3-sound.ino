
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

void myDelay(int amount) {
  while (amount > 0) {
    for (int i = 0; i < 16; i++) {
      if (buttonState[i]) {
        soundState += (i + 5) * 10;
      }
    }
    analogWrite(2, soundState & 255);
    delay(1);
    amount--;
  }
}

void loop() {
  digitalWrite(6, HIGH);
  myDelay(1);
  digitalWrite(6, LOW);
  myDelay(1);
  for (int i = 0; i < 16; i++) {
    buttonState[i] = digitalRead(5) == LOW;
    // Serial.print(buttonState ? "#" : "-");
    digitalWrite(7, LOW);
    myDelay(1);
    digitalWrite(7, HIGH);
    myDelay(1);
  }
  Serial.println();
  myDelay(10);
}