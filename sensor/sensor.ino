#define SECOND  1000
#define MINUTE 60000

#define COUNTER_PIN 2

int counter_reading = 0;

void ISR_impulse() {
  Serial.println("Pulse!");
}

void setup() {
  Serial.begin(9600);
  pinMode(COUNTER_PIN, INPUT);
  attachInterrupt(digitalPinToInterrupt(COUNTER_PIN), ISR_impulse, FALLING);
}

void loop() {
}
