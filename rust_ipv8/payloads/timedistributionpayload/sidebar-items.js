initSidebarItems({"struct":[["TimeDistributionPayload","The global time in the system. Uses a lamport clock system. Ipv8 stores global time values using, at most, 64 bits. Therefore there is a finite number of global time values available. To avoid malicious peers from quickly pushing the global time value to the point where none are left, peers will only accept messages with a global time that is within a locally evaluated limit. This limit is set to the median of the neighbors’ global time values plus a predefined margin. https://en.wikipedia.org/wiki/Lamport_timestamps (from dispersy docs. TODO: still up to date?)"]]});