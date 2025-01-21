import socket
import matplotlib.pyplot as plt
from matplotlib.animation import FuncAnimation
import collections
import json
import keyboard

# Flag to indicate whether the server should keep running
keep_running = True

# Configure the server
HOST = '0.0.0.0'
PORT = 8080

MAX_POINTS = 100

a_data = collections.deque([0] * MAX_POINTS, maxlen=MAX_POINTS)
b_data = collections.deque([0] * MAX_POINTS, maxlen=MAX_POINTS)
c_data = collections.deque([0] * MAX_POINTS, maxlen=MAX_POINTS)

fig, ax = plt.subplots()
a_line, = ax.plot(range(MAX_POINTS), a_data, label="A", color="r")
b_line, = ax.plot(range(MAX_POINTS), b_data, label="B", color="g")
c_line, = ax.plot(range(MAX_POINTS), c_data, label="C", color="b")
ax.set_ylim(-200, 200)
ax.set_xlim(0, MAX_POINTS - 1)
ax.legend(loc="upper left")
ax.set_title("Wifi data")
ax.set_xlabel("Time")
ax.set_ylabel("Values")

def update(frame, client_socket, client_address):
    global a_data, b_data, c_data

    data = client_socket.recv(1024)  # Buffer size of 1024 bytes

    if not data:
        print("Status: Package empty")
        return

    try:
        raw_data = data.decode('utf-8')

        print(f"Received from {client_address}: {raw_data}")

        clean_data = raw_data.strip()

        # Parse the JSON data
        parsed_data = json.loads(clean_data)
        header = parsed_data["HEADER"]
        a_value = parsed_data["D1"]
        b_value = parsed_data["D2"]
        c_value = parsed_data["D3"]

        # Add new values to the deque
        a_data.append(a_value)
        b_data.append(b_value)
        c_data.append(c_value)

        # Update the line data
        a_line.set_ydata(a_data)
        b_line.set_ydata(b_data)
        c_line.set_ydata(c_data)

        print("HEADER:", header, "D1:", a_value, "D2:", b_value, "D3:", c_value)
        return
    except json.JSONDecodeError:
        print("Status: Unsuccessful when decoding JSON")
    except UnicodeDecodeError:
        print("Status: Unsuccessful when Unicode decoding")

def start_server():
    global keep_running

    # Create a TCP/IP socket
    server_socket = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
    try:
        # Bind the socket to the address and port
        server_socket.bind((HOST, PORT))
        print(f"Server started on {HOST}:{PORT}")
        
        # Start listening for incoming connections
        server_socket.listen(5)  # Queue up to 5 connections
        print("Waiting for a connection...")

        server_socket.settimeout(1.0)
        
        while keep_running:
            try:
                # Accept a connection
                client_socket, client_address = server_socket.accept()
                print(f"Connection from {client_address}")
                handle_client(client_socket, client_address)

            except socket.timeout:
                if keyboard.is_pressed('q'):
                    keep_running = False
                continue
    except Exception as e:
        print(f"Error: {e}")
    finally:
        server_socket.close()

def handle_client(client_socket, client_address):
    global keep_running
    try:
        def on_close(event):
            print("Closing the application...")
            client_socket.close()
            plt.close(fig)

        fig.canvas.mpl_connect('close_event', on_close)
    
        ani = FuncAnimation(fig, update, interval=20, fargs=(client_socket, client_address,))
        plt.show()

    except ConnectionResetError:
        print(f"Client {client_address} unexpectedly disconnected.")
    finally:
        client_socket.close()

if __name__ == "__main__":
    start_server()
