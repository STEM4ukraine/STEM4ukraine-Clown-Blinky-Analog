ha:cschem-sheet-v1 {
	ha:obj_indirect.1 {
		li:objects {
			ha:group.1 {
				uuid=KfG5tqOgL1tbLJ3rbD8AAAAY;
				li:objects {
					ha:group.1 {
						uuid=KfG5tqOgL1tbLJ3rbD8AAAAZ; loclib_name=bc817_sot23;
						li:objects {
						}
						ha:attrib {
							footprint=SOT23
							li:portmap {
								{B->pcb/pinnum=1}
								{E->pcb/pinnum=2}
								{C->pcb/pinnum=3}
							}
						}
					}
					ha:group.2 {
						uuid=KfG5tqOgL1tbLJ3rbD8AAACD; loclib_name=led5;
						li:objects {
						}
						ha:attrib {
							footprint=LED5
							li:portmap {
								{C->pcb/pinnum=1}
								{A->pcb/pinnum=2}
							}
						}
					}
					ha:group.3 {
						uuid=KfG5tqOgL1tbLJ3rbD8AAACE; loclib_name=wspr_cap_p;
						li:objects {
						}
						ha:attrib {
							footprint=wspr-cap-p.lht
							li:portmap {
								{P->pcb/pinnum=1}
								{N->pcb/pinnum=2}
							}
						}
					}
					ha:group.4 {
						uuid=KfG5tqOgL1tbLJ3rbD8AAACF; loclib_name=BC547_pth;
						li:objects {
						}
						ha:attrib {
							footprint=TO92.fp
							li:portmap {
								{B->pcb/pinnum=2}
								{E->pcb/pinnum=3}
								{C->pcb/pinnum=1}
							}
						}
					}
					ha:group.5 {
						uuid=KfG5tqOgL1tbLJ3rbD8AAACG; loclib_name=3mmLED_backplane;
						li:objects {
						}
						ha:attrib {
							footprint=3mmLEDbackplane.lht
							li:portmap {
								{C->pcb/pinnum=1}
								{A->pcb/pinnum=2}
							}
						}
					}
					ha:group.6 {
						uuid=QxPegv+Z5u35HjZlhEUAAABK; loclib_name=1n4148_minimelf;
						li:objects {
						}
						ha:attrib {
							footprint=minimelf
							li:portmap {
								{C->pcb/pinnum=1}
								{A->pcb/pinnum=2}
							}
						}
					}
					ha:group.7 {
						uuid=QxPegv+Z5u35HjZlhEUAAABN; loclib_name=1n400X_pth;
						li:objects {
						}
						ha:attrib {
							footprint={alf(400mil, type=normal)}
							li:portmap {
								{C->pcb/pinnum=1}
								{A->pcb/pinnum=2}
							}
						}
					}
				}
				ha:attrib {
					ha:purpose = { value=devmap; prio=0; }
				}
			}
		}
	}
	ha:obj_direct.2 {
		uuid=KfG5tqOgL1tbLJ3rbD8AAAAC;
		li:objects {
			ha:pen.sheet-decor { shape=round; size=125; color=#777777; font_height=3000; font_family=sans; }
			ha:pen.titlebox-frame { shape=round; size=250; color=#777777; font_height=0; }
			ha:pen.titlebox-fill { shape=round; size=250; color=#bbffbb; font_height=0; }
			ha:pen.titlebox-big { shape=round; size=250; color=#777777; font_height=3000; font_family=sans; }
			ha:pen.titlebox-small { shape=round; size=250; color=#777777; font_height=1500; font_family=sans; }
			ha:pen.wire { shape=round; size=250; color=#2222bb; font_height=3000; font_family=sans; }
			ha:pen.bus { shape=round; size=1500; color=#2222bb; font_height=3000; font_family=sans; }
			ha:pen.hub { shape=round; size=3000; color=#6666ff; font_height=3000; font_family=sans; }
			ha:pen.sym-decor { shape=round; size=125; color=#119911; font_height=3000; font_family=sans; }
			ha:pen.sym-primary { shape=round; size=125; color=#119911; font_height=3000; font_family=sans; font_style=bold; }
			ha:pen.sym-secondary { shape=round; size=125; color=#33bb33; font_height=3000; font_family=sans; }
			ha:pen.term-decor { shape=round; size=250; color=#222222; font_height=3000; font_family=sans; }
			ha:pen.term-primary { shape=round; size=250; color=#222222; font_height=3000; font_family=sans; font_style=bold; }
			ha:pen.term-secondary { shape=round; size=250; color=#555555; font_height=3000; font_family=sans; }
			ha:pen.busterm-decor { shape=round; size=1500; color=#222222; font_height=3000; font_family=sans; }
			ha:pen.busterm-primary { shape=round; size=1500; color=#222222; font_height=3000; font_family=sans; font_style=bold; }
			ha:pen.busterm-secondary { shape=round; size=1500; color=#555555; font_height=3000; font_family=sans; }
			ha:pen.junction { shape=round; size=1000; color=#2222bb; font_height=3000; font_family=sans; }
			ha:group.2 {
				uuid=KfG5tqOgL1tbLJ3rbD8AAAAJ; src_uuid=iNOQfJpO6hT/HFDFGjoAAABC;
				x=55000; y=71000; rot=270.000000;
				li:objects {
					ha:group.1 {
						uuid=KfG5tqOgL1tbLJ3rbD8AAAAK; src_uuid=iNOQfJpO6hT/HFDFGjoAAABD;
						x=20000; y=0;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=-4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=-3000; y1=0; dyntext=1; stroke=term-primary; text=%../a.display/name%; }
						}
						ha:attrib {
							name=2
							role=terminal
						}
					}
					ha:group.2 {
						uuid=KfG5tqOgL1tbLJ3rbD8AAAAL; src_uuid=iNOQfJpO6hT/HFDFGjoAAABE;
						mirx=1;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=-4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=-3000; y1=0; dyntext=1; stroke=term-primary; text=%../a.display/name%; }
						}
						ha:attrib {
							name=1
							role=terminal
						}
					}
					ha:text.3 { x1=12000; y1=2000; rot=90.000000; dyntext=1; stroke=sym-primary; text=%../a.value%; floater=1; }
					ha:text.4 { x1=8000; y1=2000; rot=90.000000; dyntext=1; stroke=sym-primary; text=%../A.name%; floater=1; }
					ha:polygon.5 {
						li:outline {
							ha:line { x1=4000; y1=2000; x2=4000; y2=-2000; }
							ha:line { x1=4000; y1=-2000; x2=16000; y2=-2000; }
							ha:line { x1=16000; y1=-2000; x2=16000; y2=2000; }
							ha:line { x1=16000; y1=2000; x2=4000; y2=2000; }
						}
						stroke=sym-decor;
					}
				}
				ha:attrib {
					-sym-copyright=(C) 2022 Tibor 'Igor2' Palinkas
					-sym-license-dist=GPLv2+
					-sym-license-use=Public Domain
					-sym-source=sch-rnd default symbol lib
					footprint=acy(300)
					name=R1
					role=symbol
					value=120R
				}
			}
			ha:group.5 {
				uuid=KfG5tqOgL1tbLJ3rbD8AAABz; src_uuid=KfG5tqOgL1tbLJ3rbD8AAABs;
				x=15000; y=75000; mirx=1; miry=1;
				li:objects {
					ha:text.1 { x1=0; y1=-6000; dyntext=1; stroke=sym-primary; text=%../A.name%; floater=1; }
					ha:group.2 {
						uuid=KfG5tqOgL1tbLJ3rbD8AAAB0; src_uuid=KfG5tqOgL1tbLJ3rbD8AAABt;
						mirx=1;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=1000; y1=0; dyntext=1; stroke=term-primary; text=%../A.name%; }
						}
						ha:attrib {
							name=1
							role=terminal
						}
					}
					ha:group.3 {
						uuid=KfG5tqOgL1tbLJ3rbD8AAAB1; src_uuid=KfG5tqOgL1tbLJ3rbD8AAABu;
						x=0; y=4000; mirx=1;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=1000; y1=0; dyntext=1; stroke=term-primary; text=%../A.name%; }
						}
						ha:attrib {
							name=2
							role=terminal
						}
					}
					ha:group.4 {
						uuid=KfG5tqOgL1tbLJ3rbD8AAAB2; src_uuid=KfG5tqOgL1tbLJ3rbD8AAABv;
						x=0; y=8000; mirx=1;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=1000; y1=0; dyntext=1; stroke=term-primary; text=%../A.name%; }
						}
						ha:attrib {
							name=3
							role=terminal
						}
					}
					ha:group.5 {
						uuid=KfG5tqOgL1tbLJ3rbD8AAAB3; src_uuid=KfG5tqOgL1tbLJ3rbD8AAABw;
						x=0; y=12000; mirx=1;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=1000; y1=0; dyntext=1; stroke=term-primary; text=%../A.name%; }
						}
						ha:attrib {
							name=4
							role=terminal
						}
					}
					ha:group.6 {
						uuid=KfG5tqOgL1tbLJ3rbD8AAAB4; src_uuid=KfG5tqOgL1tbLJ3rbD8AAABx;
						x=0; y=16000; mirx=1;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=1000; y1=0; dyntext=1; stroke=term-primary; text=%../A.name%; }
						}
						ha:attrib {
							name=5
							role=terminal
						}
					}
					ha:group.7 {
						uuid=KfG5tqOgL1tbLJ3rbD8AAAB5; src_uuid=KfG5tqOgL1tbLJ3rbD8AAABy;
						x=0; y=20000; mirx=1;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=1000; y1=0; dyntext=1; stroke=term-primary; text=%../A.name%; }
						}
						ha:attrib {
							name=6
							role=terminal
						}
					}
					ha:polygon.8 {
						li:outline {
							ha:line { x1=0; y1=-2000; x2=0; y2=22000; }
							ha:line { x1=0; y1=22000; x2=4000; y2=22000; }
							ha:line { x1=4000; y1=22000; x2=4000; y2=-2000; }
							ha:line { x1=4000; y1=-2000; x2=0; y2=-2000; }
						}
						stroke=sym-decor;
					}
				}
				ha:attrib {
					footprint=TypeBpthUSBvertical.lht
					name=USB
					role=symbol
				}
			}
			ha:group.6 {
				uuid=KfG5tqOgL1tbLJ3rbD8AAACA; src_uuid=iNOQfJpO6hT/HFDFGjoAAAAQ;
				x=55000; y=31000; rot=270.000000;
				li:objects {
					ha:group.1 {
						uuid=KfG5tqOgL1tbLJ3rbD8AAACB; src_uuid=iNOQfJpO6hT/HFDFGjoAAAAR;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=-4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=-3000; y1=0; dyntext=1; stroke=term-primary; text=%../a.display/name%; }
						}
						ha:attrib {
							name=C
							role=terminal
						}
					}
					ha:group.2 {
						uuid=KfG5tqOgL1tbLJ3rbD8AAACC; src_uuid=iNOQfJpO6hT/HFDFGjoAAAAS;
						x=-16000; y=0; mirx=1;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=-4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=-3000; y1=0; dyntext=1; stroke=term-primary; text=%../a.display/name%; }
						}
						ha:attrib {
							name=A
							role=terminal
						}
					}
					ha:line.3 { x1=-4000; y1=0; x2=-6000; y2=0; stroke=sym-decor; }
					ha:line.4 { x1=-12000; y1=0; x2=-10000; y2=0; stroke=sym-decor; }
					ha:line.5 { x1=-10000; y1=4000; x2=-6000; y2=0; stroke=sym-decor; }
					ha:line.6 { x1=-6000; y1=0; x2=-10000; y2=-4000; stroke=sym-decor; }
					ha:line.7 { x1=-10000; y1=4000; x2=-10000; y2=-4000; stroke=sym-decor; }
					ha:line.8 { x1=-6000; y1=4000; x2=-6000; y2=-4000; stroke=sym-decor; }
					ha:text.10 { x1=-8000; y1=13000; rot=90.000000; dyntext=1; stroke=sym-primary; text=%../A.name%; floater=1; }
					ha:line.11 { x1=-8000; y1=8000; x2=-6000; y2=11000; stroke=sym-decor; }
					ha:line.12 { x1=-6000; y1=11000; x2=-7000; y2=10000; stroke=sym-decor; }
					ha:line.13 { x1=-6000; y1=11000; x2=-6517; y2=9545; stroke=sym-decor; }
					ha:line.14 { x1=-10000; y1=7000; x2=-8000; y2=10000; stroke=sym-decor; }
					ha:line.15 { x1=-8000; y1=10000; x2=-8000; y2=8000; stroke=sym-decor; }
					ha:line.16 { x1=-8303; y1=6354; x2=-6303; y2=9354; stroke=sym-decor; }
					ha:line.17 { x1=-6303; y1=9354; x2=-7303; y2=8354; stroke=sym-decor; }
					ha:line.18 { x1=-6303; y1=9354; x2=-6820; y2=7899; stroke=sym-decor; }
					ha:line.19 { x1=-10303; y1=5354; x2=-8303; y2=8354; stroke=sym-decor; }
					ha:line.20 { x1=-8303; y1=8354; x2=-8303; y2=6354; stroke=sym-decor; }
				}
				ha:attrib {
					-sym-copyright=(C) 2022 Tibor 'Igor2' Palinkas
					-sym-license-dist=GPLv2+
					-sym-license-use=Public Domain
					-sym-source=sch-rnd default symbol lib
					devmap=3mmLED_backplane
					name=LED1
					role=symbol
				}
			}
			ha:group.7 {
				uuid=KfG5tqOgL1tbLJ3rbD8AAACK; src_uuid=iNOQfJpO6hT/HFDFGjoAAAAQ;
				x=79000; y=31000; rot=270.000000;
				li:objects {
					ha:group.1 {
						uuid=KfG5tqOgL1tbLJ3rbD8AAACL; src_uuid=iNOQfJpO6hT/HFDFGjoAAAAR;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=-4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=-3000; y1=0; dyntext=1; stroke=term-primary; text=%../a.display/name%; }
						}
						ha:attrib {
							name=C
							role=terminal
						}
					}
					ha:group.2 {
						uuid=KfG5tqOgL1tbLJ3rbD8AAACM; src_uuid=iNOQfJpO6hT/HFDFGjoAAAAS;
						x=-16000; y=0; mirx=1;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=-4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=-3000; y1=0; dyntext=1; stroke=term-primary; text=%../a.display/name%; }
						}
						ha:attrib {
							name=A
							role=terminal
						}
					}
					ha:line.3 { x1=-4000; y1=0; x2=-6000; y2=0; stroke=sym-decor; }
					ha:line.4 { x1=-12000; y1=0; x2=-10000; y2=0; stroke=sym-decor; }
					ha:line.5 { x1=-10000; y1=4000; x2=-6000; y2=0; stroke=sym-decor; }
					ha:line.6 { x1=-6000; y1=0; x2=-10000; y2=-4000; stroke=sym-decor; }
					ha:line.7 { x1=-10000; y1=4000; x2=-10000; y2=-4000; stroke=sym-decor; }
					ha:line.8 { x1=-6000; y1=4000; x2=-6000; y2=-4000; stroke=sym-decor; }
					ha:text.10 { x1=-8000; y1=13000; rot=90.000000; dyntext=1; stroke=sym-primary; text=%../A.name%; floater=1; }
					ha:line.11 { x1=-8000; y1=8000; x2=-6000; y2=11000; stroke=sym-decor; }
					ha:line.12 { x1=-6000; y1=11000; x2=-7000; y2=10000; stroke=sym-decor; }
					ha:line.13 { x1=-6000; y1=11000; x2=-6517; y2=9545; stroke=sym-decor; }
					ha:line.14 { x1=-10000; y1=7000; x2=-8000; y2=10000; stroke=sym-decor; }
					ha:line.15 { x1=-8000; y1=10000; x2=-8000; y2=8000; stroke=sym-decor; }
					ha:line.16 { x1=-8303; y1=6354; x2=-6303; y2=9354; stroke=sym-decor; }
					ha:line.17 { x1=-6303; y1=9354; x2=-7303; y2=8354; stroke=sym-decor; }
					ha:line.18 { x1=-6303; y1=9354; x2=-6820; y2=7899; stroke=sym-decor; }
					ha:line.19 { x1=-10303; y1=5354; x2=-8303; y2=8354; stroke=sym-decor; }
					ha:line.20 { x1=-8303; y1=8354; x2=-8303; y2=6354; stroke=sym-decor; }
				}
				ha:attrib {
					-sym-copyright=(C) 2022 Tibor 'Igor2' Palinkas
					-sym-license-dist=GPLv2+
					-sym-license-use=Public Domain
					-sym-source=sch-rnd default symbol lib
					devmap=3mmLED_backplane
					name=LED2
					role=symbol
				}
			}
			ha:group.9 {
				uuid=KfG5tqOgL1tbLJ3rbD8AAACY; src_uuid=iNOQfJpO6hT/HFDFGjoAAABC;
				x=79000; y=71000; rot=270.000000;
				li:objects {
					ha:group.1 {
						uuid=KfG5tqOgL1tbLJ3rbD8AAACZ; src_uuid=iNOQfJpO6hT/HFDFGjoAAABD;
						x=20000; y=0;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=-4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=-3000; y1=0; dyntext=1; stroke=term-primary; text=%../a.display/name%; }
						}
						ha:attrib {
							name=2
							role=terminal
						}
					}
					ha:group.2 {
						uuid=KfG5tqOgL1tbLJ3rbD8AAACa; src_uuid=iNOQfJpO6hT/HFDFGjoAAABE;
						mirx=1;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=-4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=-3000; y1=0; dyntext=1; stroke=term-primary; text=%../a.display/name%; }
						}
						ha:attrib {
							name=1
							role=terminal
						}
					}
					ha:text.3 { x1=12000; y1=2000; rot=90.000000; dyntext=1; stroke=sym-primary; text=%../a.value%; floater=1; }
					ha:text.4 { x1=8000; y1=2000; rot=90.000000; dyntext=1; stroke=sym-primary; text=%../A.name%; floater=1; }
					ha:polygon.5 {
						li:outline {
							ha:line { x1=4000; y1=2000; x2=4000; y2=-2000; }
							ha:line { x1=4000; y1=-2000; x2=16000; y2=-2000; }
							ha:line { x1=16000; y1=-2000; x2=16000; y2=2000; }
							ha:line { x1=16000; y1=2000; x2=4000; y2=2000; }
						}
						stroke=sym-decor;
					}
				}
				ha:attrib {
					-sym-copyright=(C) 2022 Tibor 'Igor2' Palinkas
					-sym-license-dist=GPLv2+
					-sym-license-use=Public Domain
					-sym-source=sch-rnd default symbol lib
					footprint=acy(300)
					name=R2
					role=symbol
					value=120R
				}
			}
			ha:group.26 {
				uuid=KfG5tqOgL1tbLJ3rbD8AAACn;
				x=-9000; y=-121000;
				li:objects {
					ha:line.1 { x1=64000; y1=172000; x2=64000; y2=168000; stroke=wire; }
				}
				ha:attrib {
					ha:role = { value=wire-net; prio=0; }
				}
			}
			ha:group.29 {
				uuid=KfG5tqOgL1tbLJ3rbD8AAACo;
				x=-53000; y=-121000;
				li:objects {
					ha:line.1 { x1=132000; y1=172000; x2=132000; y2=168000; stroke=wire; }
				}
				ha:attrib {
					ha:role = { value=wire-net; prio=0; }
				}
			}
			ha:group.243 {
				uuid=QjFjMRmh3XZZy/10yEkAAACF; src_uuid=iNOQfJpO6hT/HFDFGjoAAAB6;
				x=55000; y=75000;
				li:objects {
					ha:group.1 {
						uuid=QjFjMRmh3XZZy/10yEkAAACG; src_uuid=iNOQfJpO6hT/HFDFGjoAAAB7;
						rot=270.000000;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=-4000; y2=0; stroke=term-decor; }
						}
						ha:attrib {
							ha:name = { value=1; prio=220; }
							role=terminal
						}
					}
					ha:line.2 { x1=2500; y1=4000; x2=-2500; y2=4000; stroke=sym-decor; }
					ha:text.3 { x1=-4000; y1=4000; x2=4000; y2=7000; halign=center; dyntext=0; stroke=sym-primary; text=Vcc; }
				}
				ha:attrib {
					-sym-copyright=(C) 2022 Tibor 'Igor2' Palinkas
					-sym-license-dist=GPLv2+
					-sym-license-use=Public Domain
					-sym-source=sch-rnd default symbol lib
					li:connect {
						{1:Vcc}
					}
					role=symbol
				}
			}
			ha:group.245 {
				uuid=QjFjMRmh3XZZy/10yEkAAACH; src_uuid=iNOQfJpO6hT/HFDFGjoAAAB6;
				x=27000; y=75000;
				li:objects {
					ha:group.1 {
						uuid=QjFjMRmh3XZZy/10yEkAAACI; src_uuid=iNOQfJpO6hT/HFDFGjoAAAB7;
						rot=270.000000;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=-4000; y2=0; stroke=term-decor; }
						}
						ha:attrib {
							ha:name = { value=1; prio=220; }
							role=terminal
						}
					}
					ha:line.2 { x1=2500; y1=4000; x2=-2500; y2=4000; stroke=sym-decor; }
					ha:text.3 { x1=-4000; y1=4000; x2=4000; y2=7000; halign=center; dyntext=0; stroke=sym-primary; text=Vcc; }
				}
				ha:attrib {
					-sym-copyright=(C) 2022 Tibor 'Igor2' Palinkas
					-sym-license-dist=GPLv2+
					-sym-license-use=Public Domain
					-sym-source=sch-rnd default symbol lib
					li:connect {
						{1:Vcc}
					}
					role=symbol
				}
			}
			ha:group.246 {
				uuid=QjFjMRmh3XZZy/10yEkAAACL; src_uuid=iNOQfJpO6hT/HFDFGjoAAABm;
				x=27000; y=55000;
				li:objects {
					ha:group.1 {
						uuid=QjFjMRmh3XZZy/10yEkAAACM; src_uuid=iNOQfJpO6hT/HFDFGjoAAABn;
						rot=90.000000;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=-4000; y2=0; stroke=term-decor; }
						}
						ha:attrib {
							ha:name = { value=1; prio=220; }
							role=terminal
						}
					}
					ha:line.2 { x1=-1500; y1=-5000; x2=1500; y2=-5000; stroke=sym-decor; }
					ha:line.3 { x1=-500; y1=-6000; x2=500; y2=-6000; stroke=sym-decor; }
					ha:line.4 { x1=-2500; y1=-4000; x2=2500; y2=-4000; stroke=sym-decor; }
				}
				ha:attrib {
					-sym-copyright=(C) 2022 Tibor 'Igor2' Palinkas
					-sym-license-dist=GPLv2+
					-sym-license-use=Public Domain
					-sym-source=sch-rnd default symbol lib
					li:connect {
						{1:GND}
					}
					role=symbol
				}
			}
			ha:group.247 {
				uuid=QjFjMRmh3XZZy/10yEkAAACN;
				x=-17000; y=-101000;
				li:objects {
					ha:line.1 { x1=36000; y1=156000; x2=36000; y2=164000; stroke=wire; }
					ha:line.2 { x1=36000; y1=156000; x2=44000; y2=156000; stroke=wire; }
					ha:line.6 { x1=44000; y1=156000; x2=44000; y2=172000; stroke=wire; }
					ha:line.7 { x1=48000; y1=172000; x2=44000; y2=172000; stroke=wire; }
				}
				ha:attrib {
					ha:role = { value=wire-net; prio=0; }
				}
			}
			ha:group.288 {
				uuid=QjFjMRmh3XZZy/10yEkAAACP;
				x=-33000; y=-105000;
				li:objects {
					ha:line.2 { x1=52000; y1=180000; x2=64000; y2=180000; stroke=wire; }
				}
				ha:attrib {
					ha:role = { value=wire-net; prio=0; }
				}
			}
			ha:group.327 {
				uuid=2lcqVN2rhDGCAt/8b5kAAABi; src_uuid=2lcqVN2rhDGCAt/8b5kAAABf;
				x=35000; y=75000; miry=1;
				li:objects {
					ha:text.1 { x1=0; y1=-6000; dyntext=1; stroke=sym-primary; text=%../A.name%; floater=1; }
					ha:group.2 {
						uuid=2lcqVN2rhDGCAt/8b5kAAABj; src_uuid=2lcqVN2rhDGCAt/8b5kAAABg;
						mirx=1;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=1000; y1=0; dyntext=1; stroke=term-primary; text=%../A.name%; }
						}
						ha:attrib {
							name=1
							role=terminal
						}
					}
					ha:group.3 {
						uuid=2lcqVN2rhDGCAt/8b5kAAABk; src_uuid=2lcqVN2rhDGCAt/8b5kAAABh;
						x=0; y=4000; mirx=1;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=1000; y1=0; dyntext=1; stroke=term-primary; text=%../A.name%; }
						}
						ha:attrib {
							name=2
							role=terminal
						}
					}
					ha:polygon.4 {
						li:outline {
							ha:line { x1=0; y1=-2000; x2=0; y2=6000; }
							ha:line { x1=0; y1=6000; x2=4000; y2=6000; }
							ha:line { x1=4000; y1=6000; x2=4000; y2=-2000; }
							ha:line { x1=4000; y1=-2000; x2=0; y2=-2000; }
						}
						stroke=sym-decor;
					}
				}
				ha:attrib {
					footprint=connector(1,2)
					name=5V
					role=symbol
				}
			}
			ha:group.432 {
				uuid=4MvvHQVIWeK3YvmhR68AAABL; src_uuid=iNOQfJpO6hT/HFDFGjoAAABm;
				x=55000; y=15000;
				li:objects {
					ha:group.1 {
						uuid=4MvvHQVIWeK3YvmhR68AAABM; src_uuid=iNOQfJpO6hT/HFDFGjoAAABn;
						rot=90.000000;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=-4000; y2=0; stroke=term-decor; }
						}
						ha:attrib {
							ha:name = { value=1; prio=220; }
							role=terminal
						}
					}
					ha:line.2 { x1=-1500; y1=-5000; x2=1500; y2=-5000; stroke=sym-decor; }
					ha:line.3 { x1=-500; y1=-6000; x2=500; y2=-6000; stroke=sym-decor; }
					ha:line.4 { x1=-2500; y1=-4000; x2=2500; y2=-4000; stroke=sym-decor; }
				}
				ha:attrib {
					-sym-copyright=(C) 2022 Tibor 'Igor2' Palinkas
					-sym-license-dist=GPLv2+
					-sym-license-use=Public Domain
					-sym-source=sch-rnd default symbol lib
					li:connect {
						{1:GND}
					}
					role=symbol
				}
			}
			ha:group.434 {
				uuid=4MvvHQVIWeK3YvmhR68AAABN; src_uuid=4MvvHQVIWeK3YvmhR68AAABK;
				x=55000; y=15000;
				li:objects {
					ha:line.1 { x1=0; y1=0; x2=24000; y2=0; stroke=wire; }
					ha:line.2 { x1=0; y1=16000; x2=0; y2=0; stroke=wire; }
					ha:line.3 { x1=24000; y1=16000; x2=24000; y2=0; stroke=wire; }
				}
				ha:attrib {
					ha:role = { value=wire-net; prio=0; }
				}
			}
			ha:group.440 {
				uuid=4MvvHQVIWeK3YvmhR68AAABO;
				x=-5000; y=-105000;
				li:objects {
					ha:line.1 { x1=84000; y1=180000; x2=84000; y2=176000; stroke=wire; }
					ha:line.2 { x1=60000; y1=176000; x2=60000; y2=180000; stroke=wire; }
					ha:line.3 { x1=60000; y1=180000; x2=84000; y2=180000; stroke=wire; }
				}
				ha:attrib {
					ha:role = { value=wire-net; prio=0; }
				}
			}
			ha:connection.444 {
				li:conn {
					/2/5/5/1
					/2/247/1
				}
			}
			ha:connection.445 {
				li:conn {
					/2/5/6/1
					/2/247/1
				}
			}
			ha:connection.446 {
				li:conn {
					/2/5/7/1
					/2/247/1
					/2/247/2
				}
			}
			ha:connection.447 {
				li:conn {
					/2/246/1/1
					/2/247/2
					/2/247/6
				}
			}
			ha:connection.448 {
				li:conn {
					/2/26/1
					/2/6/2/1
				}
			}
			ha:connection.449 {
				li:conn {
					/2/26/1
					/2/2/1/1
				}
			}
			ha:connection.450 {
				li:conn {
					/2/29/1
					/2/7/2/1
				}
			}
			ha:connection.451 {
				li:conn {
					/2/29/1
					/2/9/1/1
				}
			}
			ha:connection.452 {
				li:conn {
					/2/288/2
					/2/5/2/1
				}
			}
			ha:connection.453 {
				li:conn {
					/2/288/2
					/2/245/1/1
				}
			}
			ha:connection.454 {
				li:conn {
					/2/434/1
					/2/432/1/1
					/2/434/2
				}
			}
			ha:connection.455 {
				li:conn {
					/2/434/2
					/2/6/1/1
				}
			}
			ha:connection.456 {
				li:conn {
					/2/434/3
					/2/7/1/1
				}
			}
			ha:connection.457 {
				li:conn {
					/2/440/1
					/2/9/2/1
				}
			}
			ha:connection.458 {
				li:conn {
					/2/440/2
					/2/2/2/1
				}
			}
			ha:connection.459 {
				li:conn {
					/2/440/2
					/2/243/1/1
					/2/440/3
				}
			}
			ha:connection.460 {
				li:conn {
					/2/327/2/1
					/2/288/2
				}
			}
			ha:connection.461 {
				li:conn {
					/2/327/3/1
					/2/247/7
				}
			}
		}
		ha:attrib {
			drawing_min_height=200000
			drawing_min_width=287000
			maintainer=<maint. attr>
			page=<page attr>
			print_page=A/4
			title=<please set sheet title attribute>
		}
	}
  li:sch-rnd-conf-v1 {
   ha:overwrite {
    ha:editor {
     draw_grid = 1
     grids_idx = 0
     grid = 1.0240 mm
     line_refraction = false
     ha:local_grid {
      enable = 0
     }
     fullscreen = 0
    }
   }
  }
}
