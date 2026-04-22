use owo_colors::OwoColorize;

use crate::{chapter::model, menu};

pub fn content(section_title: &str, section: &str) {
  let subheaders: [model::Header; 4];
  subheaders = [
    model::Header::new("Chapter Introduction", ci_content),
    model::Header::new("Section Introduction", si_content),
    model::Header::new("Ping Command Basics", pcb_content),
    model::Header::new("Using Ping with Names and with IP Addresses", upwnawia_content),
  ];

  model::Header::prompt_header(&subheaders, section_title, section);
}


// Subheaders content below.

// Header: Chapter Introduction. Abbreviated as ci.
fn ci_content() {
  let solid_disc = "\u{2022}";

  menu::header_title("Chapter Introduction");

  println!(
  "This chapter turns our attention to routing from end-to-end across an entire enterprise network.\n\
  How do you troubleshoot an IPv4 network?\n\
  How do you verify correct operation, identify root causes, and fix those for various IP routing features?\n\
  How do you do that in the presence of an IP addressing and subnetting plan?\n\n\
  In particular, this chapter focuses on two tools and how to use them:\n\
  {solid_disc} ping\n\
  {solid_disc} and traceroute\n\n\
  Both tools test the IPv4 data plane; that is, the ability of each networking device to route or \
  forward IPv4 packets.\n\
  This chapter devotes a major section each to ping and traceroute.\n\
  The chapter then ends with a short discussion of two other router tools that can also be useful \
  for troubleshooting:\n\
  {solid_disc} Telnet\n\
  {solid_disc} and Secure Shell (SSH)
  ")
}

// Header: Section Introduction. Abbreviated as si.
fn si_content() {
  menu::header_title("Section Introduction: Problem Isolation Using the ping Command");

  println!(
  "Someone sends you an email or text, or a phone message, asking you to look into a user's network problem.\n\
  You Secure Shell (SSH) to a router and issue a {0} command that works.\n\
  What does that result rule out as a possible reason for the problem?\n\
  What does it rule in as still being a possible root cause?\n\n\
  Then you issue another {0} to another address, and this time the ping fails.\n\
  Again, what does the failure of that {0} command tell you?\n\
  What parts of IPv4 routing may still be a problem, and what parts do you now know are not a problem?\n\n\
  The {0} command gives us one of the most common network troubleshooting tools.\n\
  When the {0} command succeeds, it confirms many individual parts of how IP routing works, ruling out some possible causes \
  of the current problem.\n\
  When a {0} command fails, it often helps narrow down where in the internetwork the root cause of the problem may be \
  happening, further isolating the problem.\n\n\
  This section begins with a brief explanation of how {0} works.\n\
  It then moves on to some suggestions and analysis of how to use the {0} command to isolate problems by removing \
  some items from consideration.
  ",
  "ping".bright_yellow().bold()
  )
}

// Header: Ping Command Basics. Abbreviated as pcb.
fn pcb_content() {
  let solid_disc = "\u{2022}";

  menu::header_title("Ping Command Basics");

  println!(
  "The {0} command tests connectivity by sending packets to an IP address, expecting the device at \
  that address to send packets back.\n\
  The command sends packets that mean “if you receive this packet, and it is addressed to you, send a reply back.”\n\
  Each time the {0} command sends one of these packets and receives the message sent back by the other host, the \
  {0} command knows a packet made it from the source host to the destination and back.\n\n\
  More formally, the {0} command uses the Internet Control Message Protocol (ICMP), specifically the ICMP echo request \
  and ICMP echo reply messages.\n\
  ICMP defines many other messages as well, but these two messages were made specifically for connectivity testing by 
  commands like {0}.\n\
  As a protocol, ICMP does not rely on TCP or UDP, and it does not use any application layer protocol.\n\
  It functions as part of Layer 3, as a control protocol to assist IP by helping manage the IP network functions.\n\n\
  The {0} command is supported on many different devices and many common operating systems.\n\
  The command has many options:\n\
  {solid_disc} the name or IP address of the destination,\n\
  {solid_disc} how many times the command should send an echo request,\n\
  {solid_disc} how long the command should wait (timeout) for an echo reply, \n\
  {solid_disc} how big to make the packets, and many other options.\n\n\
  Take a moment to review the output of an IOS {0} command.\n\
  By default, the Cisco IOS {0} command sends five echo messages, with a timeout of 2 seconds.\n\
  If the command does not receive an echo reply within 2 seconds, the command considers that message to be a failure, \
  and the command lists a period.\n\
  If a successful reply is received within 2 seconds, the command displays an exclamation point.\n\
  So, in this first command, the first echo reply timed out, whereas the other four received a matching echo reply within 2 seconds.\n\n\
  The {0} command has common and normal behavior: it usualy shows one failure to start, but then the rest of the messages work.\n\
  This usually happens because some device in the end-to-end route is missing an ARP table entry.
  ",
  "ping".bright_yellow().bold(),
  );

  println!(
  "{0}\n\n\
  {solid_disc} The {1} command uses the Internet Control Message Protocol (ICMP), specifically the ICMP echo request \
  and ICMP echo reply messages made specifically for connectivity testing.\n\
  {solid_disc} ICMP defines many other messages as well.\n\
  {solid_disc} As a protocol, ICMP does not rely on TCP or UDP, and it does not use any application layer protocol.\n\
  It functions as part of Layer 3, as a control protocol to assist IP by helping manage the IP network functions.\n\
  {solid_disc} The {1} command is supported on many different devices and many common operating systems.\n\
  {solid_disc} The command has many options: the name or IP address of the destination, how many times the command should \
  send an echo request, how long the command should wait (timeout) for an echo reply, how big to make the packets, \
  and many other options.\n\
  {solid_disc} The {1} command has an attribure called timeout (measured in seconds) that it uses to time an \
  ICMP echo reply. If the command does not receive a reply within the default amount of time (usually 2 seconds) \
  then the command will consider the message (the ICMP echo request and reply conversation) a failure.\n\
  {solid_disc} The {0} command has common and normal behavior: it usualy shows one failure to start, but then the rest \
  of the messages work.This usually happens because some device in the end-to-end route is missing an ARP table entry.
  ",
  "REMEMBER".bright_white().bold(),
  "ping".bright_yellow().bold()
  )
}

// Header: Using Ping with Names and with IP Addresses. Abbreviated as upwnawia.
fn upwnawia_content() {
  menu::header_title("Using Ping with Names and with IP Addresses");

  println!(
  "All the ping examples so far in this chapter show a ping of an IP address. However, the {0} command can use hostnames, \
  and pinging a hostname allows the network engineer to further test whether the Domain Name System (DNS) process works.\n\n\
  First, most every TCP/IP application today uses hostnames rather than IP addresses to identify the other device.\n\
  No one opens a web browser and types in 72.163.4.185.\n\
  Instead, they type in a web address, like www.cisco.com, which includes the hostname www.cisco.com.\n\
  Then, before a host can send data to a specific IP address, the host must first ask a DNS server to resolve that hostname \
  into the matching IP address.\n\n\
  See: Figure 18-10 on page 428 for network diagram example.\n\n\
  When troubleshooting, testing from the host by pinging using a hostname can be very helpful.\n\
  The command, of course, tests the host's own DNS client settings.\n\
  For instance, a classic comparison is to first ping the destination host using the hostname, which requires a DNS request.\n\
  Then, repeat the same test, but use the destination host's IP address instead of its name, which does not require the DNS request.\n\
  If the ping of the hostname fails but the ping of the IP address works, the problem usually has something to do with DNS.
  ",
  "ping".bright_yellow().bold()
  )
}