// https://www.midi.org/specifications-old/item/gm-level-1-sound-set

#[rustfmt::skip]
pub const GENERAL_MIDI: &[((u16, u16), &str)] = &[
    ((  0,   0), "Acoustic Grand Piano"   ),
    ((  0,   1), "Bright Acoustic Piano"  ),
    ((  0,   2), "Electric Grand Piano"   ),
    ((  0,   3), "Honky-tonk Piano"       ),
    ((  0,   4), "Electric Piano 1"       ),
    ((  0,   5), "Electric Piano 2"       ),
    ((  0,   6), "Harpsichord"            ),
    ((  0,   7), "Clavi"                  ),
    ((  0,   8), "Celesta"                ),
    ((  0,   9), "Glockenspiel"           ),
    ((  0,  10), "Music Box"              ),
    ((  0,  11), "Vibraphone"             ),
    ((  0,  12), "Marimba"                ),
    ((  0,  13), "Xylophone"              ),
    ((  0,  14), "Tubular Bells"          ),
    ((  0,  15), "Dulcimer"               ),
    ((  0,  16), "Drawbar Organ"          ),
    ((  0,  17), "Percussive Organ"       ),
    ((  0,  18), "Rock Organ"             ),
    ((  0,  19), "Church Organ"           ),
    ((  0,  20), "Reed Organ"             ),
    ((  0,  21), "Accordion"              ),
    ((  0,  22), "Harmonica"              ),
    ((  0,  23), "Tango Accordion"        ),
    ((  0,  24), "Acoustic Guitar (nylon)"),
    ((  0,  25), "Acoustic Guitar (steel)"),
    ((  0,  26), "Electric Guitar (jazz)" ),
    ((  0,  27), "Electric Guitar (clean)"),
    ((  0,  28), "Electric Guitar (muted)"),
    ((  0,  29), "Overdriven Guitar"      ),
    ((  0,  30), "Distortion Guitar"      ),
    ((  0,  31), "Guitar harmonics"       ),
    ((  0,  32), "Acoustic Bass"          ),
    ((  0,  33), "Electric Bass (finger)" ),
    ((  0,  34), "Electric Bass (pick)"   ),
    ((  0,  35), "Fretless Bass"          ),
    ((  0,  36), "Slap Bass 1"            ),
    ((  0,  37), "Slap Bass 2"            ),
    ((  0,  38), "Synth Bass 1"           ),
    ((  0,  39), "Synth Bass 2"           ),
    ((  0,  40), "Violin"                 ),
    ((  0,  41), "Viola"                  ),
    ((  0,  42), "Cello"                  ),
    ((  0,  43), "Contrabass"             ),
    ((  0,  44), "Tremolo Strings"        ),
    ((  0,  45), "Pizzicato Strings"      ),
    ((  0,  46), "Orchestral Harp"        ),
    ((  0,  47), "Timpani"                ),
    ((  0,  48), "String Ensemble 1"      ),
    ((  0,  49), "String Ensemble 2"      ),
    ((  0,  50), "SynthStrings 1"         ),
    ((  0,  51), "SynthStrings 2"         ),
    ((  0,  52), "Choir Aahs"             ),
    ((  0,  53), "Voice Oohs"             ),
    ((  0,  54), "Synth Voice"            ),
    ((  0,  55), "Orchestra Hit"          ),
    ((  0,  56), "Trumpet"                ),
    ((  0,  57), "Trombone"               ),
    ((  0,  58), "Tuba"                   ),
    ((  0,  59), "Muted Trumpet"          ),
    ((  0,  60), "French Horn"            ),
    ((  0,  61), "Brass Section"          ),
    ((  0,  62), "SynthBrass 1"           ),
    ((  0,  63), "SynthBrass 2"           ),
    ((  0,  64), "Soprano Sax"            ),
    ((  0,  65), "Alto Sax"               ),
    ((  0,  66), "Tenor Sax"              ),
    ((  0,  67), "Baritone Sax"           ),
    ((  0,  68), "Oboe"                   ),
    ((  0,  69), "English Horn"           ),
    ((  0,  70), "Bassoon"                ),
    ((  0,  71), "Clarinet"               ),
    ((  0,  72), "Piccolo"                ),
    ((  0,  73), "Flute"                  ),
    ((  0,  74), "Recorder"               ),
    ((  0,  75), "Pan Flute"              ),
    ((  0,  76), "Blown Bottle"           ),
    ((  0,  77), "Shakuhachi"             ),
    ((  0,  78), "Whistle"                ),
    ((  0,  79), "Ocarina"                ),
    ((  0,  80), "Lead 1 (square)"        ),
    ((  0,  81), "Lead 2 (sawtooth)"      ),
    ((  0,  82), "Lead 3 (calliope)"      ),
    ((  0,  83), "Lead 4 (chiff)"         ),
    ((  0,  84), "Lead 5 (charang)"       ),
    ((  0,  85), "Lead 6 (voice)"         ),
    ((  0,  86), "Lead 7 (fifths)"        ),
    ((  0,  87), "Lead 8 (bass + lead)"   ),
    ((  0,  88), "Pad 1 (new age)"        ),
    ((  0,  89), "Pad 2 (warm)"           ),
    ((  0,  90), "Pad 3 (polysynth)"      ),
    ((  0,  91), "Pad 4 (choir)"          ),
    ((  0,  92), "Pad 5 (bowed)"          ),
    ((  0,  93), "Pad 6 (metallic)"       ),
    ((  0,  94), "Pad 7 (halo)"           ),
    ((  0,  95), "Pad 8 (sweep)"          ),
    ((  0,  96), "FX 1 (rain)"            ),
    ((  0,  97), "FX 2 (soundtrack)"      ),
    ((  0,  98), "FX 3 (crystal)"         ),
    ((  0,  99), "FX 4 (atmosphere)"      ),
    ((  0, 100), "FX 5 (brightness)"      ),
    ((  0, 101), "FX 6 (goblins)"         ),
    ((  0, 102), "FX 7 (echoes)"          ),
    ((  0, 103), "FX 8 (sci-fi)"          ),
    ((  0, 104), "Sitar"                  ),
    ((  0, 105), "Banjo"                  ),
    ((  0, 106), "Shamisen"               ),
    ((  0, 107), "Koto"                   ),
    ((  0, 108), "Kalimba"                ),
    ((  0, 109), "Bag pipe"               ),
    ((  0, 110), "Fiddle"                 ),
    ((  0, 111), "Shanai"                 ),
    ((  0, 112), "Tinkle Bell"            ),
    ((  0, 113), "Agogo"                  ),
    ((  0, 114), "Steel Drums"            ),
    ((  0, 115), "Woodblock"              ),
    ((  0, 116), "Taiko Drum"             ),
    ((  0, 117), "Melodic Tom"            ),
    ((  0, 118), "Synth Drum"             ),
    ((  0, 119), "Reverse Cymbal"         ),
    ((  0, 120), "Guitar Fret Noise"      ),
    ((  0, 121), "Breath Noise"           ),
    ((  0, 122), "Seashore"               ),
    ((  0, 123), "Bird Tweet"             ),
    ((  0, 124), "Telephone Ring"         ),
    ((  0, 125), "Helicopter"             ),
    ((  0, 126), "Applause"               ),
    ((  0, 127), "Gunshot"                ),
    ((128,   0), "Standard Drum Kit"      ),
];
