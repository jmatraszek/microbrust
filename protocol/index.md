---
layout: page
title: Reverse engineering Arturia Microbrute MIDI SysEx protocol
---

Reverse engineering Arturia Microbrute MIDI SysEx protocol
===================

## Why?
Arturia decided to release its software only for Windows and OS X, so as a Linux user I could not use my Microbrute to the fullest... There was no other way than write an application myself, but in order to do that, I had to figure out how Arturia's software and Microbrute communicate.

## How?
First of all I needed a Windows machine (or a Windows running on a VirtualBox) with [USBPcap](http://desowin.org/usbpcap/) installed and an [Arturia's Microbrute](https://www.arturia.com/microbrute/resources) application. To analyze captured packets I used, obviously, [Wireshark](https://www.wireshark.org/). I knew that it communicates through USB, so I had to reverse engineer the protocol... and that's where the fun begun.

### Prepare
I setup a capture process on a Windows machine as described on [USBPcap's website](http://desowin.org/usbpcap/tour.html). Step 2 contains a really useful tip I omitted at first: after identifying the root hub you will sniff on, disconnect the device, start sniffing and then connect the device. This will capture the USB enumeration (more on USB enumeration can be found in chapter 4 [here](http://s.eeweb.com/members/mark_harrington/answers/1333179451-USB_Complete_3rdEdition.pdf)) packets that are used for proper packet dissection in Wireshark. So I started sniffing, connected the device, launched the application and changed some settings.

### Getting hands dirty
A quick look at the captured packets in Wireshark and I discovered that Microbrute uses MIDI SysEx protocol. It's nothing special for a synthesizer, after all. Knowing that I could take a closer look. I used `tshark` with `sysex` filter to analyze captured packets and show only important data. First column represents the source, second the host and last one is the SysEx data.

```
$ tshark -r file_with_captured_packets.pcapng -2 -R 'sysex' -T fields \
  -e usb.src -e usb.dst -e usbaudio.sysex.reassembled.data
host    2.5.2   f0:7e:7f:06:01:f7
2.5.5   host    f0:7e:01:06:02:00:20:6b:04:00:02:01:01:00:03:02:f7
host    2.5.2   f0:00:20:6b:05:01:00:00:06:f7
2.5.5   host    f0:00:20:6b:05:01:00:01:05:00:00:00:00:00:00:00:00:00:f7
host    2.5.2   f0:00:20:6b:05:01:01:00:08:f7
2.5.5   host    f0:00:20:6b:05:01:01:01:07:00:00:00:00:00:00:00:00:00:f7
host    2.5.2   f0:00:20:6b:05:01:02:00:35:f7
2.5.5   host    f0:00:20:6b:05:01:02:01:34:01:00:00:00:00:00:00:00:01:f7
host    2.5.2   f0:00:20:6b:05:01:03:00:10:f7
2.5.5   host    f0:00:20:6b:05:01:03:01:0f:01:00:00:00:00:00:00:00:01:f7
host    2.5.2   f0:00:20:6b:05:01:04:00:2f:f7
2.5.5   host    f0:00:20:6b:05:01:04:01:2e:00:00:00:00:00:00:00:00:00:f7
host    2.5.2   f0:00:20:6b:05:01:05:00:0c:f7
2.5.5   host    f0:00:20:6b:05:01:05:01:0b:00:00:00:00:00:00:00:00:00:f7
host    2.5.2   f0:00:20:6b:05:01:06:00:0e:f7
2.5.5   host    f0:00:20:6b:05:01:06:01:0d:00:00:00:00:00:00:00:00:00:f7
host    2.5.2   f0:00:20:6b:05:01:07:00:12:f7
2.5.5   host    f0:00:20:6b:05:01:07:01:11:02:01:00:00:00:00:00:00:00:f7
host    2.5.2   f0:00:20:6b:05:01:08:00:33:f7
2.5.5   host    f0:00:20:6b:05:01:08:01:32:00:00:00:00:00:00:00:00:00:f7
host    2.5.2   f0:00:20:6b:05:01:09:00:2d:f7
2.5.5   host    f0:00:20:6b:05:01:09:01:2c:02:01:00:00:00:00:00:00:00:f7
host    2.5.2   f0:00:20:6b:05:01:0a:00:39:f7
2.5.5   host    f0:00:20:6b:05:01:0a:01:38:04:02:00:00:00:00:00:00:00:f7
host    2.5.2   f0:00:20:6b:05:01:0b:00:37:f7
2.5.5   host    f0:00:20:6b:05:01:0b:01:36:03:01:00:00:00:00:00:00:01:f7
host    2.5.2   f0:00:20:6b:05:01:0c:00:2b:f7
2.5.5   host    f0:00:20:6b:05:01:0c:01:2a:01:00:00:00:00:00:00:00:01:f7
host    2.5.2   f0:00:20:6b:05:01:0d:00:3d:f7
2.5.5   host    f0:00:20:6b:05:01:0d:01:3c:00:00:00:00:00:00:00:00:00:f7
host    2.5.2   f0:00:20:6b:05:01:0e:01:0b:01:f7
host    2.5.2   f0:00:20:6b:05:01:0f:01:0b:02:f7
host    2.5.2   f0:00:20:6b:05:01:10:01:0b:00:f7
host    2.5.2   f0:00:20:6b:05:01:11:01:11:01:f7
[...some more similar packets here...]
host    2.5.2   f0:00:20:6b:05:01:57:01:0d:01:f7
host    2.5.2   f0:00:20:6b:05:01:58:01:0d:00:f7
host    2.5.2   f0:00:20:6b:05:01:59:01:0f:00:f7
host    2.5.2   f0:00:20:6b:05:01:5a:01:0f:01:f7
```

I noticed that the packets can be divided into two groups. In the first one for every outgoing packet there is one packet sent back by Microbrute. The second group contains only outgoing packets. Doing a live capture I easily noticed that the first group appears just after running the application (so this is possibly reading the current state of the device), while the packets from the second group appear after changing any of the device's parameter (so they are used to alter the state of the device). At first I focused on the second group, as it seemed easier to reverse engineer.

### Writing to Microbrute
I started a live capture, applied a `sysex` filter in Wireshark and observed what happend. I captured a couple of packets -- the first group. Then, when I started changing values in the application there will be one packet appearing after one change of a parameter. We should add a comment to the packet about what parameter and to what value was changed. Then, we can inspect captured packets using `thsark` (the last column show the comment added during live capture session):

```$ tshark -r microbrute2comments.pcapng -2 -R 'sysex' -T fields \
  -e usb.src -e usb.dst -e usbaudio.sysex.reassembled.data -e frame.comment
[first group of packets here]
host    2.5.2   f0:00:20:6b:05:01:0e:01:0b:01:f7        Set note priority to "low"
host    2.5.2   f0:00:20:6b:05:01:0f:01:0b:02:f7        Set note priority to "high"
host    2.5.2   f0:00:20:6b:05:01:10:01:0b:00:f7        Set note priority to "last"
host    2.5.2   f0:00:20:6b:05:01:11:01:34:02:f7        Set seq retrig to "none"
host    2.5.2   f0:00:20:6b:05:01:12:01:34:00:f7        Set seq retrig to "reset"
host    2.5.2   f0:00:20:6b:05:01:13:01:34:01:f7        Set seq retrig to "legato"
```

And this is something we can work on! Let's dissect an example packet: we know it's MIDI SysEx and this packets is expected to start with `f0` and end with `f7`. Later on we know that SysEx message should contain manufacturer ID. Those numbers can be found [here](https://www.midi.org/specifications/item/manufacturer-id-numbers). Arturia has been assigned a number `0x00 0x20 0x6b` and in fact: those are three bytes following `f0`. The next byte, `0x05` is a number assigned to a specific device (in our case Microbrute). When I closely examined sixth and eighth byte I noticed that it is always set to `0x01`. Seventh byte is incremented by one with each consecutive SysEx command sent to Microbrute, so its purpose is to be a counter.

Ninth and tenth bytes are the crucial ones. Ninth is always set to the same value if we change the value of the same parameter (e.g. `0x0b` for Note priority or `0x34` for Seq retrig), so it has to be the code which defines the parameter. Tenth byte then must hold the value of a changed parameter. For example: for note priority we can easily assign `0x00` to "Last", `0x01` to "Low" and `0x02` and "High". The SysEx message sent to the Microbrute has the following format:

```
f0:00:20:6b:05:01:XX:01:YY:ZZ:f7
```

where `XX` is the current value of the counter, `YY` is the byte defining the parameter and `ZZ` is the byte with the value. Then I had to patiently click through all of the options to be able to assign proper values for all parameters and values.

### Reading from Microbrute
After I have sorted out how to change the Microbrute setting's, I had to focus on reverse engineering the first part of the packets. This would allow me to read the current Microbrute's state. Take a look at the first part of the sniffed packets:

```
$ tshark -r microbrute -2 -R 'sysex' -T fields \
  -e usb.src -e usb.dst -e usbaudio.sysex.reassembled.data
host    2.4.2   f0:7e:7f:06:01:f7
2.4.5   host    f0:7e:01:06:02:00:20:6b:04:00:02:01:01:00:03:02:f7
host    2.4.2   f0:00:20:6b:05:01:00:00:06:f7
2.4.5   host    f0:00:20:6b:05:01:00:01:05:00:00:00:00:00:00:00:00:00:f7
host    2.4.2   f0:00:20:6b:05:01:01:00:08:f7
2.4.5   host    f0:00:20:6b:05:01:01:01:07:00:00:00:00:00:00:00:00:00:f7
host    2.4.2   f0:00:20:6b:05:01:02:00:35:f7
2.4.5   host    f0:00:20:6b:05:01:02:01:34:01:00:00:00:00:00:00:00:01:f7
host    2.4.2   f0:00:20:6b:05:01:03:00:10:f7
2.4.5   host    f0:00:20:6b:05:01:03:01:0f:01:00:00:00:00:00:00:00:01:f7
host    2.4.2   f0:00:20:6b:05:01:04:00:2f:f7
2.4.5   host    f0:00:20:6b:05:01:04:01:2e:00:00:00:00:00:00:00:00:00:f7
host    2.4.2   f0:00:20:6b:05:01:05:00:0c:f7
2.4.5   host    f0:00:20:6b:05:01:05:01:0b:00:00:00:00:00:00:00:00:00:f7
host    2.4.2   f0:00:20:6b:05:01:06:00:0e:f7
2.4.5   host    f0:00:20:6b:05:01:06:01:0d:00:00:00:00:00:00:00:00:00:f7
host    2.4.2   f0:00:20:6b:05:01:07:00:12:f7
2.4.5   host    f0:00:20:6b:05:01:07:01:11:02:01:00:00:00:00:00:00:00:f7
host    2.4.2   f0:00:20:6b:05:01:08:00:33:f7
2.4.5   host    f0:00:20:6b:05:01:08:01:32:00:00:00:00:00:00:00:00:00:f7
host    2.4.2   f0:00:20:6b:05:01:09:00:2d:f7
2.4.5   host    f0:00:20:6b:05:01:09:01:2c:02:01:00:00:00:00:00:00:00:f7
host    2.4.2   f0:00:20:6b:05:01:0a:00:39:f7
2.4.5   host    f0:00:20:6b:05:01:0a:01:38:04:02:00:00:00:00:00:00:00:f7
host    2.4.2   f0:00:20:6b:05:01:0b:00:37:f7
2.4.5   host    f0:00:20:6b:05:01:0b:01:36:03:01:00:00:00:00:00:00:01:f7
host    2.4.2   f0:00:20:6b:05:01:0c:00:2b:f7
2.4.5   host    f0:00:20:6b:05:01:0c:01:2a:01:00:00:00:00:00:00:00:01:f7
host    2.4.2   f0:00:20:6b:05:01:0d:00:3d:f7
2.4.5   host    f0:00:20:6b:05:01:0d:01:3c:00:00:00:00:00:00:00:00:00:f7
```

It is easy to notice that the first packet is significantly shorter than the fourteen following packets sent by host. Again, when I consulted [MIDI SysEx specification](https://www.midi.org/specifications/item/table-4-universal-system-exclusive-messages) I came to the conclusion it is the request for the device to identify itself (`0x06 0x01` -- fourth and fifth byte of the first byte sent to the device -- is described in the table as "General information: identify request", while `0x06 0x02` -- fourth and fifth byte sent by the device to the host -- is described in the table as "General information: identify reply").

I have failed in reverse engineering the meaning of the response (`f0:7e:01:06:02:00:20:6b:04:00:02:01:01:00:03:02:f7`), but it is always the same, regardless of Microbrute's state, so it cannot be used to respone with the current state. This must mean that the following 14 packets pairs (request-reply) are used to obtain Microbrute's state... wait, there's 14 parameters in the configuration -- that's a perfect match!

Taking a closer look at the packets pairs I noticed that they contain the "usual" stuff: `0xf0` as the first byte, then three bytes of Arturia's identifier (`0x00 0x20 0x6b`), a byte of Microbrute's identifier (`0x05`), probably a delimiter (`0x01`) and a counter. First seven bytes sorted out. Eight is always `0x00` in the request and `0x01` in the response.

For the request there is only one byte left (except the usual ending byte: `0xf7`). After comparing ninth bytes in request and response packets I realized that in the request it is always greater by `1` than in the response... moreover the value of this byte in the response is the same as we used to specify the parameter in the requests we used to set its values!

For example: the value identifying "Note priority" parameter is `0x0b`. This means that it is required to set the ninth byte of the request to `0x0c`. Requests for reading Microbrute data are fully reverse engineered now. I just needed to change the parameters a couple of times, request the data from Microbrute and sniff the packets to see how they changed. Doing that I observed that in the response the tenth byte is changing which indicated it has the values of a given parameter. The eight following bytes, from eleventh to eighteenth, are quite weird, as they usually have value `0x00`, only sometimes something different. Maybe it's some kind of checksum, but for now they seem insignificant.

## Summary

The request to obtain Microbrute's parameter's value has the following format:

```
f0:00:20:6b:05:01:XX:00:YY:f7
```

where `XX` is the counter and `YY` is the parameter code incremented by `1`, while the response has the following format:

```
f0:00:20:6b:05:01:XX:01:YY:ZZ:00:00:00:00:00:00:00:00:f7
```

where `XX` is the counter, `YY` is the parameter code (not incremented!) and `ZZ` is the parameter's value.

I have prepare the full documentation of the communication between Microbrute and the application and it can be found [here](sysex.pdf).

