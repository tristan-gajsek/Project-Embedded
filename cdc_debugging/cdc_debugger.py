import serial
import time
import msvcrt
import keyboard

# Configure the serial connection
ser = serial.Serial(
    port='COM13',
    baudrate=115200,
    timeout=1
)


# Send data as bytes
def send_data(data):
    ser.write(data.encode())
    print(f"Sent: {data}")

def read_response(response_count=1):
    infinite = response_count == -1
    while True:
        if keyboard.is_pressed('q'):
            print("Exiting response reading by force")
            break
        if ser.in_waiting > 0:
            raw_response = ser.read_all()
            print(f"Raw Response: {raw_response}")
            if raw_response:
                response = raw_response.decode().strip()
                print(f"Received: {response}")
                if (not infinite): 
                    response_count -= 1

            if response_count <= 0 and not infinite:
                if ser.in_waiting > 0: #Try to get one more response in case it didn't all go trough
                    raw_response = ser.read_all()
                    print(f"Raw Response: {raw_response}")
                    if raw_response:
                        response = raw_response.decode().strip()
                        print(f"Received: {response}")
                break

def read_response_inf():
    while True:
        if ser.in_waiting > 0:
            raw_response = ser.read_all()
            print(f"Raw Response: {raw_response}")
            if raw_response:
                response = raw_response.decode().strip()
                print(f"Received: {response}")


def main():
    print("CLI za komunikacijo STM32 - ESP32")
    print("Na voljo ukazi: ON, OFF, IP, MULTI, WEB, RESET, HELP, EXIT")

    while True:
        cmd = input("Vnesite ukaz: ").strip().upper()
        if cmd == "EXIT":
            print("Izhod...")
            break
        if cmd == "INIT":
            print("Zahteva po inicializaciji poslana...")
            send_data("A")
            #time.sleep(2)
            read_response(3)
            continue
        if cmd == "ON":
            print("Zahteva po inicializaciji poslana...")
            send_data("A") # TODO: Change from A
            #time.sleep(2)
            read_response()
            continue
        if cmd == "OFF":
            print("Zahteva za zagon strežnika poslana...")
            send_data("B")
            #time.sleep(2)
            read_response()
            continue
        if cmd == "IP":
            print("Zahteva po strani poslana...")
            send_data("C")
            #time.sleep(2)
            read_response()
            continue
        if cmd == "MULTI":
            print("Zahteva za test poslana...")
            send_data("D")
            #time.sleep(2)
            read_response()
            continue
        if cmd == "WEB":
            print("Zahteva za test poslana...")
            send_data("E")
            #time.sleep(2)
            read_response()
            continue
        if cmd == "RESET":
            print("Zahteva za test poslana...")
            send_data("F")
            #time.sleep(2)
            read_response()
            continue
        if cmd == "MODE":
            print("Zahteva za preklopitev načina...")
            send_data("G")
            read_response_inf()
        if cmd == "PASSTHROUGH":
            print("Zahteva za passthrough...")
            send_data("H")
            read_response_inf()
        if cmd == "TEST":
            print("Zahteva za test poslana...")
            send_data("X")
            read_response()
            continue
        if cmd == "IPCONFIG":
            print("Zahteva za omrezne informacije ESP-ja")
            send_data("Y")
            read_response()
            continue
        if cmd == "SEND":
            print("Zahteva za posiljanje podatkov poslana...")
            send_data("Z")
            read_response(-1)
            continue
        if cmd == "HELP":
            print("Na voljo ukazi: INIT, START, REQUEST, HELP, EXIT")
            print("INIT: Initializes the connection of ESP32")
            print("START: Starts the webserver")
            print("REQUEST: Sends a request to the server")
            print("EXIT: Exits the CLI")
            continue
        if cmd == "CLEAN":
            read_response(-1)
            continue

if __name__ == "__main__":
    main()