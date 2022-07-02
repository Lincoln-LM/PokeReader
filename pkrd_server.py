from enum import Enum
from http.server import BaseHTTPRequestHandler, HTTPServer
from pprint import pprint

class Button(Enum):
    A = 1,
    B = 2,
    Select = 4,
    Start = 8,
    Dright = 16,
    Dleft = 32,
    Dup = 64,
    Ddown = 128,
    R = 256,
    L = 512,
    X = 1024,
    Y = 2048,

class Handler(BaseHTTPRequestHandler):
    def _set_response(self):
        self.send_response(200)
        self.send_header('Content-type', 'text/html')
        self.end_headers()

    def do_POST(self):
        post_data = self.rfile.read(int(self.headers['Content-Length']))
        print(post_data.decode('utf-8'))
        advances = []
        buttons = []
        def press_at(advance, button: Button, double = False):
            advances.append((advance,advance + 1) if double else (advance,))
            buttons.append(button)
            if double:
                return struct.pack("LL", advance, button.value[0] ^ 0xFFF) + struct.pack("LL", advance + 1, button.value[0] ^ 0xFFF)
            return struct.pack("LL", advance, button.value[0] ^ 0xFFF)
        self._set_response()
        import struct
        final = int(input("Final A press (> 1661): "))
        self.wfile.write(
            press_at(60, Button.A) # enter/exit save screen
          + press_at(350, Button.X, True) # open menu (double because we can be on odd or even)
          + press_at(380, Button.Ddown, True) # move to bag option (^)
          + press_at(400, Button.A, True) # open bag (^)
          + press_at(450 + (final & 1), Button.X) # close bag on specific odd/even according to final A press
          + press_at(620 + (final & 1), Button.A) # get through dialogue
          + press_at(680 + (final & 1), Button.A)
          + press_at(820 + (final & 1), Button.A)
          + press_at(880 + (final & 1), Button.A)
          + press_at(940 + (final & 1), Button.A)
          + press_at(1000 + (final & 1), Button.A)
          + press_at(1060 + (final & 1), Button.A)
          + press_at(1120 + (final & 1), Button.A)
          + press_at(1600 + (final & 1), Button.A)
          + press_at(final, Button.A) # recieve fossil mon
        )
        pprint(tuple(zip(advances, buttons)))

    def log_message(self, format, *args):
        # silence logs
        return

def run(server_class=HTTPServer, handler_class=Handler, port=8000):
    server_address = ('', port)
    httpd = server_class(server_address, handler_class)
    try:
        httpd.serve_forever()
    except KeyboardInterrupt:
        pass
    httpd.server_close()

if __name__ == '__main__':
    run()