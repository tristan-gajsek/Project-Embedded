import socket
import threading
import tkinter as tk
from tkinter import messagebox
import time

# Constants
ESP_IP = "192.168.4.1"
ESP_PORT = 80
HOST = '0.0.0.0'
PORT = 8081
keep_running = True


# Function to send initial "request" to ESP
def send_initial_request():
    try:
        with socket.socket(socket.AF_INET, socket.SOCK_STREAM) as client_socket:
            client_socket.connect((ESP_IP, ESP_PORT))
            time.sleep(1)
            client_socket.send("REQUEST".encode())
            print("Sent 'request' to ESP")
    except Exception as e:
        print(f"Failed to send 'request': {e}")
    finally:
        client_socket.close()


# Function to handle incoming data from ESP
def handle_client(client_socket, client_address):
    global keep_running
    try:
        while keep_running:
            data = client_socket.recv(1024)
            if not data:
                break
            message = data.decode('utf-8')
            print(f"Received from ESP: {message}")

            if message == "giveme":
                start_gui()
    except ConnectionResetError:
        print("Connection reset by ESP")
    finally:
        client_socket.close()


# Function to start the server
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

        server_socket.settimeout(100.0)

        while keep_running:
            try:
                # Accept a connection
                client_socket, client_address = server_socket.accept()
                print(f"Connection from {client_address}")

                client_thread = threading.Thread(
                    target=handle_client, args=(client_socket, client_address), daemon=True
                )
                client_thread.start()
            except socket.timeout:
                continue
    except Exception as e:
        print(f"Server Error: {e}")
    finally:
        server_socket.close()

# Function to send credentials to ESP
def send_credentials(ssid, password):
    try:
        with socket.socket(socket.AF_INET, socket.SOCK_STREAM) as client_socket:
            client_socket.connect((ESP_IP, ESP_PORT))
            time.sleep(1)
            message = f"{ssid}:{password}"
            client_socket.send(message.encode())
            print("Credentials sent successfully")
    except Exception as e:
        print(f"Failed to send credentials: {e}")


# GUI function to get SSID and password from the user
def start_gui():
    def on_submit():
        ssid = ssid_entry.get()
        password = password_entry.get()
        if ssid and password:
            send_credentials(ssid, password)
            gui_root.destroy()
        else:
            messagebox.showwarning("Input Error", "Please enter both SSID and password")

    gui_root = tk.Tk()
    gui_root.title("Send WiFi Credentials")

    tk.Label(gui_root, text="SSID:").grid(row=0, column=0, padx=10, pady=10)
    ssid_entry = tk.Entry(gui_root)
    ssid_entry.grid(row=0, column=1, padx=10, pady=10)

    tk.Label(gui_root, text="Password:").grid(row=1, column=0, padx=10, pady=10)
    password_entry = tk.Entry(gui_root, show="*")
    password_entry.grid(row=1, column=1, padx=10, pady=10)

    tk.Button(gui_root, text="Submit", command=on_submit).grid(row=2, column=0, columnspan=2, pady=10)
    gui_root.mainloop()


# Main function
if __name__ == "__main__":
    send_initial_request()
    time.sleep(1)
    start_server()