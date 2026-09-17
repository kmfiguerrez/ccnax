use dioxus::prelude::*;

use crate::{
    components::{
        GreenNote, KeyTopic, my_accordion::{Accordion, AccordionContent, AccordionItem, AccordionTrigger}
    }, utils::{h3_heading, h4_heading}
};

#[component]
pub fn Content() -> Element {
    rsx! {
        p { class: "mb-4",
            strong { "The STP/RSTP algorithm creates a spanning tree of interfaces that forward frames." }
            br {}
            "The tree structure of forwarding interfaces creates a single path to and from each Ethernet link, just
            like you can trace a single path in a living, growing tree from the base of the tree to each
            leaf."
        }

        GreenNote {
            p {
                strong { "NOTE" }
                " STP was created before LAN switches even existed, using LAN bridges to connect
                LANs."
                " Today, switches play the same role as bridges, implementing STP/RSTP."
                " However, many STP/RSTP terms still refer to bridge. For the purposes of STP/RSTP and this chapter,
                consider the terms bridge and switch synonymous."
            }
        }

        {h3_heading("Spanning-tree algorithm")}
        p { class: "mb-4",
            "The process used by STP, sometimes called the "
            i { "spanning-tree algorithm" }
            " (STA), chooses
            the interfaces that should be placed into a forwarding state."
            br {}
            "For any interfaces not chosen to be in a forwarding state, STP/RSTP places the interfaces in blocking state."
            br {}
            "In other words, STP/RSTP simply picks which interfaces should forward, and any interfaces left over go to a
            blocking state."
        }

        p { "STP/RSTP uses three criteria to choose whether to put an interface in forwarding state:" }
        ol { class: "list-disc list-inside mb-4",
            li {
                "STP/RSTP elects a root switch. STP puts all working interfaces on the root switch in forwarding state."
            }
            li {
                "Each nonroot switch considers one of its ports to have the least administrative cost
                between itself and the root switch. "
                strong { "The cost (STP/RSTP cost) is called that switch's root cost" }
                " . STP/RSTP 
                places its port that is part of the least root cost path, called that switch's root port (RP),
                in forwarding state."
            }
            li {
                "Many switches can attach to the same Ethernet segment, but due to the fact that links
                connect two devices, a link would have at most two switches. With two switches on a
                link, the switch's port with the lowest root cost, as compared with the other switches attached
                to the same link, is placed in forwarding state. That switch is the designated switch, and
                that switch's interface, attached to that segment, is called the "
                i { "designated port" }
                " (DP)."
            }
        }

        GreenNote {
            p {
                strong { "NOTE" }
                " The real reason the root switches place all working interfaces in a forwarding state
                (at step 1 in the list) is that all its interfaces on the root switch will become DPs."
                " However, it is easier to just remember that all the root switches' working interfaces will forward frames."
            }
        }

        p { class: "mb-4",
            "Table 9-3 summarizes the reasons STP/RSTP places a port in forwarding or blocking state."
        }

        KeyTopic {}
        img {
            class: "mb-4 rounded-lg",
            alt: "Table 9-3 STP/RSTP: Reasons for Forwarding or Blocking",
            loading: "lazy",
            src: asset!("/assets/static/v1p3c9s1sh3t9-3.png", AssetOptions::image().with_avif()),
        }

        GreenNote {
            p {
                strong { "NOTE" }
                " STP/RSTP only considers working interfaces (those in a connected state). Failed
                interfaces (for example, interfaces with no cable installed) or administratively shutdown
                interfaces are instead placed into an STP/RSTP disabled state. So, this section uses the term
                working ports to refer to interfaces that could forward frames if STP/RSTP placed the interface into a 
                forwarding state."
            }
        }

        GreenNote {
            p {
                strong { "NOTE" }
                " STP and RSTP do differ slightly in the use of the names of some states like blocking
                and disabled, with RSTP using the status term discarding. However, those minor differences
                do not change the meaning of the discussions in this first section of the chapter. 
                The upcoming section titled “Comparing STP and RSTP” discusses these differences, both important
                and minor."
            }
        }

        Accordion { class: "mb-4",
            AccordionItem {
                AccordionTrigger { {h3_heading("The STP Bridge ID and Hello BPDU")} }
                AccordionContent {
                    p { class: "mb-4",
                        strong { "The STA begins with an election of one switch to be the root switch." }
                        br {}
                        "To better understand this election process, you need to understand the STP/RSTP messages sent 
                        between switches as well as the concept and format of the identifier used to uniquely identify 
                        each switch."
                    }

                    {h4_heading("STP/RSTP bridge ID")}
                    p { class: "mb-4",
                        strong { "The STP/RSTP bridge ID (BID) is an 8-byte value unique to each switch." }
                        br {}
                        "The bridge ID consists of a 2-byte priority field and a 6-byte system ID, with the system ID being 
                        based on a universal (burned-in) MAC address in each switch."
                        br {}
                        "Using a burned-in MAC address ensures that each switch's bridge ID will be unique."
                    }

                    {h4_heading("STP/RSTP BPDU")}
                    p { class: "mb-4",
                        strong {
                            "STP/RSTP defines messages called bridge protocol data units (BPDU), also called configuration BPDUs, 
                        which switches use to exchange information with each other."
                        }
                        br {}
                        "The most common BPDU, called a Hello BPDU, lists many details, including the sending switch's BID."
                        br {}
                        "By listing its own unique BID, switches can tell which switch sent which Hello BPDU."
                        br {}
                        "Table 9-4 lists some of the key information in the Hello BPDU."
                    }

                    KeyTopic {}
                    img {
                        class: "mb-4 rounded-lg",
                        alt: "Table 9-4 Fields in the STP Hello BPDU",
                        loading: "lazy",
                        src: asset!("/assets/static/v1p3c9s1sh3t9-4.png", AssetOptions::image().with_avif()),
                    }

                    p { class: "mb-4",
                        "For the time being, just keep the first three items from Table 9-4 in mind as the following
                        sections work through the three steps in how STP/RSTP chooses the interfaces to place into
                        a forwarding state."
                        br {}
                        "Next, the text examines the three main steps in the STP/RSTP process."
                    }
                }
            }
            AccordionItem {
                AccordionTrigger { {h3_heading("Electing the Root Switch")} }
                AccordionContent {
                    p { class: "mb-4",
                        strong { "Switches elect a root switch based on the BIDs in the BPDUs." }
                        br {}
                        "The root switch is the switch with the lowest numeric value for the BID."
                        br {}
                        "Because the two-part BID starts with the priority value, essentially the switch with the lowest 
                        priority becomes the root."
                        br {}
                        "For example, if one switch has priority 4096, and another switch has priority 8192, the switch with 
                        priority 4096 wins, regardless of what MAC address was used to create the BID for each switch."
                    }

                    {h4_heading("Lowest priority values Tiebraker")}
                    p { class: "mb-4",
                        "If a tie occurs based on the priority portion of the BID, the switch with the lowest MAC
                        address portion of the BID is the root."
                        br {}
                        "No other tiebreaker should be needed because switches use one of their own universal 
                        (burned-in) MAC addresses as the second part of their BIDs."
                        br {}
                        "So if the priorities tie, and one switch uses a MAC address of 0200.0000.0000 as
                        part of the BID and the other uses 0811.1111.1111, the first switch (MAC 0200.0000.0000)
                        becomes the root switch."
                    }

                    {h4_heading("The better (lower) BID")}
                    p { class: "mb-4",
                        "STP/RSTP elects a root switch in a manner not unlike a political election."
                        br {}
                        strong {
                            "The process begins with all switches claiming to be the root by sending Hello BPDUs listing their 
                            own BID as the root BID."
                        }
                        br {}
                        strong {
                            "If a switch hears a Hello that lists a better (lower) BID, that switch stops advertising itself as root 
                            and starts forwarding the superior Hello."
                        }
                        br {}
                        "The Hello sent by the better switch lists the better switch's BID as the root."
                        br {}
                        "It works like a political race in which a less-popular candidate gives up and leaves the race, 
                        throwing his support behind the more popular candidate."
                        br {}
                        "Eventually, everyone agrees which switch has the best (lowest) BID, and everyone supports the elected 
                        switch—which is where the political race analogy falls apart."
                    }

                    GreenNote {
                        p {
                            strong { "NOTE" }
                            " A better Hello, meaning that the listed root's BID is better (numerically lower), is
                            called a "
                            i { "superior Hello" }
                            "; a worse Hello, meaning that the listed root's BID is not as good
                            (numerically higher), is called an "
                            i { "inferior Hello." }
                        }
                    }

                    {h4_heading("The Root switch electrion process")}
                    p { class: "mb-4",
                        "Figure 9-3 shows the beginning of the root election process."
                        br {}
                        "In this case, SW1 has advertised itself as root, as have SW2 and SW3."
                        "However, SW2 now believes that SW1 is a better root, so SW2 is now forwarding the Hello 
                        originating at SW1."
                        br {}
                        "So, at this point, the figure shows SW1 is saying Hello, claiming to be root; SW2 agrees and is 
                        forwarding SW1's Hello that lists SW1 as root; but SW3 is still claiming to be best, sending its 
                        own Hello BPDUs, listing SW3's BID as the root."
                    }

                    img {
                        class: "mb-4 rounded-lg",
                        alt: "Figure 9-3 Beginnings of the Root Election Process",
                        loading: "lazy",
                        src: asset!("/assets/static/v1p3c9s1sh3f9-3.png", AssetOptions::image().with_avif()),
                    }

                    p { class: "mb-4",
                        "Two candidates still exist in Figure 9-3: SW1 and SW3."
                        br {}
                        "So, who wins? Well, from the BID, the lower-priority switch wins; if a tie occurs, the 
                        lower MAC address wins."
                        br {}
                        "As shown in the figure, SW1 has a lower BID (32769:0200.0001.0001) than SW3 (32769:0200.0003.0003), 
                        so SW1 wins, and SW3 now also believes that SW1 is the better switch."
                        br {}
                        "Figure 9-4 shows the resulting Hello messages sent by the switches."
                    }

                    p {
                        "Summarizing, the root election happens through each switch claiming to be root, with the
                        best switch being elected based on the numerically lowest BID."
                        br {}
                        "Breaking down the BID into
                        its components, the comparisons can be made as"
                    }
                    KeyTopic {}
                    ul { class: "list-disc list-inside mb-4",
                        li { "The lowest priority" }
                        li { "If that ties, the lowest switch MAC address" }
                    }

                    img {
                        class: "mb-4 rounded-lg",
                        alt: "Figure 9-4 SW1 Wins the Election",
                        loading: "lazy",
                        src: asset!("/assets/static/v1p3c9s1sh3f9-4.png", AssetOptions::image().with_avif()),
                    }
                }
            }
            AccordionItem {
                AccordionTrigger { {h3_heading("Choosing Each Switch's Root Port")} }
                AccordionContent {
                    p { class: "mb-4",
                        strong {
                            "The second part of the STP/RSTP process occurs when each nonroot switch chooses its one
                        and only "
                        }
                        i { "root port" }
                        "."
                        br {}
                        "A switch's RP is its interface through which it has the least STP/RSTP
                        cost to reach the root switch (least root cost)."
                    }

                    {h4_heading("STP/RSTP cost")}
                    p { class: "mb-4",
                        "The idea of a switch's cost to reach the root switch can be easily seen for humans."
                        br {}
                        "Just look at a network diagram that shows the root switch, lists the STP/RSTP cost associated with
                        each switch port, and identifies the nonroot switch in question."
                        br {}
                        "Switches use a different process than looking at a network diagram, of course, but using a diagram 
                        can make it easier to learn the idea."
                    }

                    p { class: "mb-4",
                        "Figure 9-5 shows just such a figure, with the same three switches shown in the last several
                        figures."
                        br {}
                        "SW1 has already won the election as root, and the figure considers the cost from
                        SW3's perspective. (Note that the figure uses some nondefault cost settings.)"
                    }

                    p { class: "mb-4",
                        "SW3 has two possible physical paths to send frames to the root switch: the direct path to
                        the left and the indirect path to the right through switch SW2."
                        br {}
                        strong {
                            "The cost is the sum of the costs of all the switch ports the frame would exit"
                        }
                        " if it flowed 
                        over that path."
                        br {}
                        "(The calculation ignores the inbound ports.) As you can see, the cost over the direct path out SW3's
                        G0/1 port has a total cost of 5, and the other path has a total cost of 8."
                        br {}
                        strong {
                            "SW3 picks its G0/1 port as root port because it is the port that is part of the least-cost path to 
                            send frames to the root switch."
                        }
                    }

                    img {
                        class: "mb-4 rounded-lg",
                        alt: "Figure 9-5 How a Human Might Calculate STP/RSTP Cost from SW3 to the Root (SW1)",
                        loading: "lazy",
                        src: asset!("/assets/static/v1p3c9s1sh3f9-5.png", AssetOptions::image().with_avif()),
                    }

                    {h4_heading("STP/RSTP port cost")}
                    p { class: "mb-4",
                        "Switches come to the same conclusion but using a different process."
                        br {}
                        "Instead, they add their local interface STP/RSTP cost to the root cost listed in each received 
                        Hello BPDU."
                        br {}
                        strong {
                            "The STP/RSTP port cost is simply an integer value assigned to each interface, per VLAN, for the
                            purpose of providing an objective measurement that allows STP/RSTP to choose which interfaces to add to 
                            the STP/RSTP topology."
                        }
                        br {}
                        "The switches also look at their neighbor's root cost, as announced in Hello BPDUs received from each 
                        neighbor."
                    }

                    {h4_heading("Calculation of STP/RSTP best (least) root cost")}
                    p { class: "mb-4",
                        "Figure 9-6 shows an example of how switches calculate their best root cost and then choose
                        their root port, using the same topology and STP/RSTP costs as shown in Figure 9-5."
                        br {}
                        "STP/RSTP on SW3 calculates its cost to reach the root over the two possible paths by adding
                        the advertised cost (in Hello messages) to the interface costs listed in the figure."
                    }

                    img {
                        class: "mb-4 rounded-lg",
                        alt: "Figure 9-5 How a Human Might Calculate STP/RSTP Cost from SW3 to the Root (SW1)",
                        loading: "lazy",
                        src: asset!("/assets/static/v1p3c9s1sh3f9-6.png", AssetOptions::image().with_avif()),
                    }

                    p { class: "mb-4",
                        "Focus on the process for a moment. The root switch sends Hellos, with a listed root cost of
                        0."
                        br {}
                        "The idea is that the root's cost to reach itself is 0."
                    }

                    p { class: "mb-4",
                        "Next, look on the left of the figure."
                        br {}
                        "SW3 takes the received cost (0) from the Hello sent by
                        SW1 and adds the interface cost (5) of the interface on which that Hello was received."
                        br {}
                        "SW3 calculates that the cost to reach the root switch, out that port (G0/1), is 5."
                    }

                    p { class: "mb-4",
                        "On the right side, SW2 has realized its best cost to reach the root is cost 4."
                        br {}
                        "So, when SW2 forwards the Hello toward SW3, SW2 lists a root cost 4."
                        br {}
                        "SW3's STP/RSTP port cost on port G0/2 is 4, so SW3 determines a total cost to reach root out 
                        its G0/2 port of 8."
                    }

                    p { class: "mb-4",
                        "As a result of the process depicted in Figure 9-6, SW3 chooses Gi0/1 as its RP because the
                        cost to reach the root switch through that port (5) is lower than the other alternative (Gi0/2,
                        cost 8)."
                        br {}
                        "Similarly, SW2 chooses Gi0/2 as its RP, with a cost of 4 (SW1's advertised cost of
                        0 plus SW2's Gi0/2 interface cost of 4)."
                        br {}
                        "Each switch places its root port into a forwarding state."
                    }

                    {h4_heading("Best root cost tiebrakers")}
                    p {
                        "Switches need a tiebreaker to use in case the best root cost ties for two or more paths."
                        br {}
                        "If a tie occurs, the switch applies these three tiebreakers to the paths that tie, in order, as follows:"
                    }
                    ol { class: "list-decimal list-inside pl-1 mb-4",
                        li { "Choose based on the lowest neighbor bridge ID." }
                        li { "Choose based on the lowest neighbor port priority." }
                        li { "Choose based on the lowest neighbor internal port number." }
                    }
                
                }
            }
            AccordionItem {
                AccordionTrigger { {h3_heading("Choosing the Designated Port on Each LAN Segment")} }
                AccordionContent {
                    p { class: "mb-4",
                        "STP/RSTP's final step to choose the STP/RSTP topology is to choose the designated port on
                        each LAN segment."
                        br {}
                        strong {
                            "The designated port (DP) on each LAN segment is the switch port that advertises the lowest-cost Hello 
                            onto a LAN segment."
                        }
                        br {}
                        strong {
                            "When a nonroot switch forwards a Hello, the nonroot switch sets the root cost field in the Hello to that 
                            switch's cost to reach the root."
                        }
                        br {}
                        "In effect, the switch with the lower cost to reach the root, among all switches connected to a segment, 
                        becomes the DP on that segment."
                    }

                    p { class: "mb-4",
                        "For example, earlier Figure 9-4 shows in bold text the parts of the Hello messages from both
                        SW2 and SW3 that determine the choice of DP on that segment."
                        br {}
                        "Note that both SW2 and SW3 list their respective cost to reach the root switch 
                        (cost 4 on SW2 and cost 5 on SW3)."
                        br {}
                        "SW2 lists the lower cost, so SW2's Gi0/1 port is the designated port on that LAN segment."
                    }

                    p { class: "mb-4",
                        "All DPs are placed into a forwarding state; so in this case, SW2's Gi0/1 interface will be in a
                        forwarding state."
                    }

                    {h4_heading("Advertised costs tiebraker")}
                    p { class: "mb-4",
                        "If the advertised costs tie, the switches break the tie by choosing the switch with the lower BID."
                        br {}
                        "In this case, SW2 would also have won, with a BID of 32769:0200.0002.0002 versus
                        SW3's 32769:0200.0003.0003."
                    }

                    GreenNote {
                        p {
                            strong { "NOTE" }
                            " Two additional tiebreakers are needed in some cases, although these would be
                            unlikely today."
                            " A single switch can connect two or more interfaces to the same collision
                            domain by connecting to a hub."
                            " In that case, the one switch hears its own BPDUs."
                            " So, if a switch ties with itself, two additional tiebreakers are used: the lowest interface 
                            STP/RSTP priority and, if that ties, the lowest internal interface number."
                        }
                    }

                    p { class: "mb-4",
                        "The only interface that does not have a reason to be in a forwarding state on the three
                        switches in the examples shown in Figures 9-3 through 9-6 is SW3's Gi0/2 port."
                        br {}
                        "So, the STP/RSTP process is now complete. Table 9-5 outlines the state of each port and shows why
                        it is in that state."
                    }

                    img {
                        class: "mb-4 rounded-lg",
                        alt: "Table 9-5 State of Each Interface",
                        loading: "lazy",
                        src: asset!("/assets/static/v1p3c9s1sh3t9-5.png", AssetOptions::image().with_avif()),
                    }

                    p { class: "mb-4",
                        "Note that the examples in this section focus on the links between the switches, but "
                        strong {
                            "switch ports connected to endpoint devices should become DPs and settle into a forwarding state."
                        }
                        br {}
                        "Working through the logic, each switch will forward BPDUs on each port as part of the
                        process to determine the DP on that LAN."
                        br {}
                        strong {
                            "Endpoints should ignore those messages because
                        they do not run STP/RSTP, so the switch will win and become DP on every access port."
                        }
                    }
                
                }
            }
        }

        {h3_heading("RECAP")}
        ol { class: "list-disc list-inside",
            li { "The STP/RSTP algorithm creates a spanning tree of interfaces that forward frames." }
            li { "STP was created before LAN switches even existed, using LAN bridges to connect LANs." }
            li {
                "The process used by STP, sometimes called the "
                i { "spanning-tree algorithm" }
                " (STA), chooses the interfaces that should be placed into a forwarding state."
                " For any interfaces not chosen to be in a forwarding state, STP/RSTP places the interfaces in blocking state."
            }
            li {
                "STP/RSTP elects a root switch, and STP puts all working interfaces on the root switch in forwarding state."
            }
            li { "Root cost is the least administrative cost to reach the root switch." }
            li {
                "A nonroot switch's port that has the least root cost is called root port (RP) and in forwarding state."
            }
            li {
                "Designated port is the port on a designated switch on one side of a link with the lowest root cost, 
                as compared with the other switch attached to the same link, and is placed in forwarding state."
            }
            li {
                "A root switch (designated switch) interfaces are designated ports and are always in a forwarding state."
            }
            li {
                "STP/RSTP only considers working interfaces (those in a connected state). Failed interfaces 
                (for example, interfaces with no cable installed) or administratively shutdown interfaces are instead placed 
                into an STP/RSTP disabled state."
            }
            li {
                "The STP/RSTP bridge ID (BID) is an 8-byte value unique to each switch. The bridge ID consists of a 2-byte 
                priority field and a 6-byte system ID, with the system ID being based on a universal (burned-in) MAC address 
                in each switch."
            }
            li {
                "STP/RSTP defines messages called bridge protocol data units (BPDU), also called configuration BPDUs, which 
                switches use to exchange information with each other."
            }
            li {
                "The most common BPDU, called a Hello BPDU, lists many details, including the sending switch's BID."
            }
            li {
                "Switches elect a root switch based on the BIDs in the BPDUs. The root switch is the switch with the 
                lowest numeric value for the BID."
            }
            li {
                "If a tie occurs based on the priority portion of the BID, the switch with the lowest MAC address portion of 
                the BID is the root."
            }
            li { "The better BID is the lower BID." }
            li { "The better Hello BPDU (lists better BID) is called the superior Hello." }
            li { "Each nonroot switch has only one root port." }
            li {
                "A switch's RP is its interface through which it has the least STP/RSTP cost to reach the root switch 
                (least root cost)."
            }
            li {
                "The STP/RSTP cost (also called root cost) is the sum of the costs of all the switch ports the frame would exit 
                to reach the root switch. So each port has a root cost."
            }
            li {
                "The STP/RSTP port cost is simply an integer value assigned to each interface, per VLAN, for the purpose of 
                providing an objective measurement that allows STP/RSTP to choose which interfaces to add to the STP/RSTP topology."
            }
            li {
                "The root switch sends Hellos, with a listed root cost of 0.
                The idea is that the root's cost to reach itself is 0."
            }
            li {
                "The designated port (DP) on each LAN segment is the switch port that advertises the lowest-cost Hello onto a 
                LAN segment."
            }
            li {
                "Switch ports connected to endpoint devices should become DPs and settle into a forwarding state."
                " Endpoints should ignore those messages because they do not run STP/RSTP, so the switch will win and 
                become DP on every access port"
            }
        }

    }
}