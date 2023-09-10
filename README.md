If you have any ideas or know of anything I should avoid or be careful with, don't hesitate to create a discussion to tell me :D

# Idea

Personally, I enjoy playing with PlayStation controllers, simply because of their almost symmetrical layout.
The problem is that only one platform supports one of the PS controllers. Only the PS4 natively supports the PS4 controller, and only the PS5 natively supports the PS5 controller. 
With this project I want to solve this problem by using the raspi in between to translate the used controller for the connected platform.
I expect there will be some problems as Playstations communicate both ways, but maybe this can be ignored...


# What I can test
- **Platforms:** PS4 and PC
- **Controllers:** PS5, PS4 connected via BT to Raspi
 
# Used hardware
- Raspberry Pi Zero 2W
  - Since this raspi is able to run the normal raspi OS, this project is made easier because the bluetooth and usb connection is handled by the OS, which means the data can just be read from and written to a filehandler instead of having to connect everything manually.
- [Raspberry Sense HAT (not yet)](https://www.raspberrypi.com/products/sense-hat)
- Emulator stick
  - Connects to PS5 controllers and emulates them as XBOX for Win10 and PS4 for PS4
  - Why don't I just use this?
    - When playing coop with a friend on a PS4, this stick often loses connection with my PS5 controller <br>
(The stick or controller is not faulty, because when playing alone the connection lasts until the controller battery dies)

# Goal
Due to the PS4 problem described in "Used Hardware", the Raspi must be connected to the targeted platform via a USB cable.

Ideally, the Raspi would support a Bluetooth connection to two controllers, processing both inputs and translating them to the platform. Since USB allows a lot of things in one connection this shouldn't really be a problem :D

# Possible Inspirations / Mapping Sources
- [GP2040](https://github.com/FeralAI/GP2040)
- [GP2040-CE](https://github.com/OpenStickCommunity/GP2040-CE)
- [Passing Link](https://github.com/passinglink/passinglink)

Why dont I just use these?
1. I want to learn how this can be done
2. Both firmwares dont support PS5 controllers
