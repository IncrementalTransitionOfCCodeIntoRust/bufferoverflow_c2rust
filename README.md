The initial C program was designed to produce a buffer overflow when typing in a password longer than 16 characters. 

Try it out by entering: "0123456789abcdefg" 
This will let the 'g' overflow onto the next array (password[16]), even though it is initialized and const. 

In your second attempt, you can simply enter "g" to see the message "access granted!". 

In this example we show that a transpilation with c2rust does not solve this issue and we can produce the exact same bufferflow. So even though our code is now transpiled into Rust, it suffers from the same obvious weakness as the initial C program. 

