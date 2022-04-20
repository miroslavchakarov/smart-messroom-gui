
This is the 2004 LCD screen and HX711 ADC up and running on Rust and Raspberry Pi.

More about the system:

This is a system called Smart Messroom, built in 2018 on Raspberry Pi and Windows IoT Core. It is now being rebuilt and will soon be able to run on Rust for a high level of protection and speed. 
Note: The system is not a weight scale! It measures food all the time. After a certain amount of food has been lifted from the food container, it starts calculating the difference in weight between NOW and the time the weight started to differ (food to be taken).
It also has a 7-inch LCD screen.

See more on the video below:

https://1drv.ms/v/s!Ak5sft2RFM38jb4jOWfsavIP3ROe2A?e=IfUv9y

(Note: Give it a few seconds to load as onedrive is a bit slower.)


As per this moment it can show the load cell ADC data on the 2004 LCD screen. (It will get more complex in time.)

Update: It can now show not only raw values but also the values converted in grams and kilograms!

Update 14.04: It can now show the quantity of food taken from the weight scale.

Update 18.04: We now have a working GUI!
Update 20.04: it can now reset the amount if customer changed his mind

The near-future goals for the Smart Messroom are (TODO):

 - (DONE!) To make the ADC (Analogue-to-Digital-Converter) and the 2004 LCD screen work with Rust and RPI2. (See other two repos)
 - (DONE! Needs improvement) Raise an event when worker picks up food from the food container and assign the difference in grams to the current customer.
 - Add new customer and display the customer number, product and quantity of food for the current client on the LCD 2004 screen.
 - Ability to reset and start measurement from the beginning in case quantity is not appropriate.


The long-term goals for this system are the following:
 - Calibration funcion (tara) while program running. (Currently the calibration value is hard coded)
 - (DONE!) Having a touch-friendly GUI on the 7-inch Touchscreen for the worker to operate with the device. (Currently testing the fltk-rs library.)
 - Having a payment system integrated, eventually with cryptocurrency or fiat currency.

