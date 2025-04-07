# My algorithm in Rust
This is a private project.


## Notes
Install https://code.visualstudio.com/docs/languages/rust

## TODOs
- [x] Protect best node from pruning
- [x] Redo pruning on max_pos after change
- [x] Node collapsing: avoid node repeating "inline". Instead of check recursively like done in python I can touch/verify the element on that the tree pruning stops. To be checked also the new nodes when I'm leaving them.
- [x] Tree mode
- [x] Add alignment recostruction
- [x] Fix print_alignment if last part is equal
- [x] Edit DE syntax in order to use […/…] for alternatives
- [x] Ragionare bene sulla gestione del pruning nel caso de: sia quando fermarlo sia su che elemento di partenza usare nel caso di salto righe
- [x] Update print_alignment in order to use hops when necessary - Evaluate if don't skip nodes in last row of alternative
- [ ] Check to use max_value if it is in last line, too
- [ ] Check if remove branching line from dont_skip is ok
- [x] Create a "drawer" in Python for thesis images
- [ ] Check images for DE already in latex with last version
- [ ] Forse migliorabile: non è necessario memorizzare i nodi della diga precedente ad uno split?
- [ ] Extensive tests on what I did until now
- [ ] Fix the disalignment between max_memory statistics and the number of nodes reported (which is not the max but the last)
- [ ] Update thesis wrt updates in code
- [ ] Non-branching nodes: move them in a dedicated structure property?
- [ ] Exclude from computation nodes that can't get a better result than the best?
- [ ] Change computation order to diagonals and parallelize them?

## "Benchmark"

Matrix size 2_885 x 23_550 = 67_941_750

Windows, not optimized compilation:
Tree size  | Versione   | RAM (approx)
----------:|:-----------|-------------:
67_941_750 | 1 (full)   | ~11gb (win, non opt)
119_072 | 2 (pruning)   | ~130mb (win, non opt)
18_658 | 3 (collapsing) | ~35mb (win, non opt)
4_886 | 4 (tree mode)   | ~19mb (win, non opt)

Linux, optimized compilation:
Tree size  | Versione   | RAM (measured)          | time
----------:|:-----------|------------------------:|-----
67_941_750 | 1 (full)   | 10_998_636kb (lin, opt) | 46.83s
119_072 | 2 (pruning)   | 51_556kb (lin, opt)     | 34.33s
18_658 | 3 (collapsing) | 6_684kb (lin, opt)      | 27.17s
4_886 | 4 (tree mode)   | 6_124kb (lin, opt)      | 24.85s

Note 1: RAM in the first table is measured as approximation during non-optimized execution with cargo.

Note 2: m is 2_884 and m^2 is 8_317_456. n+m is 26_433

Note 3: Alignment Score is 1_431 and max_pos is 15_919_429



====== ONLY TREE

        Command being timed: "target/release/thesis_rust"
        User time (seconds): 24.85
        System time (seconds): 0.12
        Percent of CPU this job got: 99%
        Elapsed (wall clock) time (h:mm:ss or m:ss): 0:25.20
        Average shared text size (kbytes): 0
        Average unshared data size (kbytes): 0
        Average stack size (kbytes): 0
        Average total size (kbytes): 0
        Maximum resident set size (kbytes): 6124
        Average resident set size (kbytes): 0
        Major (requiring I/O) page faults: 0
        Minor (reclaiming a frame) page faults: 601
        Voluntary context switches: 74
        Involuntary context switches: 121
        Swaps: 0
        File system inputs: 0
        File system outputs: 0
        Socket messages sent: 0
        Socket messages received: 0
        Signals delivered: 0
        Page size (bytes): 4096
        Exit status: 0

== FULL PATH, FULL OPTIMIZATION

        Command being timed: "target/release/thesis_rust"
        User time (seconds): 27.17
        System time (seconds): 0.10
        Percent of CPU this job got: 99%
        Elapsed (wall clock) time (h:mm:ss or m:ss): 0:27.55
        Average shared text size (kbytes): 0
        Average unshared data size (kbytes): 0
        Average stack size (kbytes): 0
        Average total size (kbytes): 0
        Maximum resident set size (kbytes): 6684
        Average resident set size (kbytes): 0
        Major (requiring I/O) page faults: 0
        Minor (reclaiming a frame) page faults: 804
        Voluntary context switches: 69
        Involuntary context switches: 207
        Swaps: 0
        File system inputs: 0
        File system outputs: 0
        Socket messages sent: 0
        Socket messages received: 0
        Signals delivered: 0
        Page size (bytes): 4096
        Exit status: 0

=========== FULL PATH, NO COLLAPSING

        Command being timed: "target/release/thesis_rust"
        User time (seconds): 34.33
        System time (seconds): 0.18
        Percent of CPU this job got: 98%
        Elapsed (wall clock) time (h:mm:ss or m:ss): 0:35.14
        Average shared text size (kbytes): 0
        Average unshared data size (kbytes): 0
        Average stack size (kbytes): 0
        Average total size (kbytes): 0
        Maximum resident set size (kbytes): 51556
        Average resident set size (kbytes): 0
        Major (requiring I/O) page faults: 0
        Minor (reclaiming a frame) page faults: 2763
        Voluntary context switches: 136
        Involuntary context switches: 1661
        Swaps: 0
        File system inputs: 0
        File system outputs: 0
        Socket messages sent: 0
        Socket messages received: 0
        Signals delivered: 0
        Page size (bytes): 4096
        Exit status: 0

======= NO PRUNING

        Command being timed: "target/release/thesis_rust"
        User time (seconds): 46.83
        System time (seconds): 34.69
        Percent of CPU this job got: 84%
        Elapsed (wall clock) time (h:mm:ss or m:ss): 1:36.87
        Average shared text size (kbytes): 0
        Average unshared data size (kbytes): 0
        Average stack size (kbytes): 0
        Average total size (kbytes): 0
        Maximum resident set size (kbytes): 10998636
        Average resident set size (kbytes): 0
        Major (requiring I/O) page faults: 345555
        Minor (reclaiming a frame) page faults: 4132446
        Voluntary context switches: 230942
        Involuntary context switches: 840
        Swaps: 0
        File system inputs: 1885648
        File system outputs: 0
        Socket messages sent: 0
        Socket messages received: 0
        Signals delivered: 0
        Page size (bytes): 4096
        Exit status: 0



========= On windows

Row j=23500 tree is 0%
Current physical memory usage: 7438336
Current virtual memory usage: 4755456

========= On linux

Row j=23540 tree is 0%
Current physical memory usage: 6209536
Current virtual memory usage: 7327744






==================================
============== NOTE ==============
==================================

(1) Come si ricorderà, mi piacerebbe fare un articolo dalla sua tesi e sottometterlo a ICTCS 2025 (scadenza 15 giugno), vedi https://ictcs2025.unich.it. Penso che per quello, il contributo principale sarà l'algoritmo di allineamento di ED-stringhe. Anche per questa ragione, mi piacerebbe avere un confronto tra il nuovo algoritmo e quello esistente. Se non mi ricordo male, avevamo detto che il nuovo è più efficiente (in termini di spazio, o anche di tempo?) asistoticamente. O mi ricordo male? Per adesso non vedo nessuna analisi teorica nel Capitolo 4. 

-->

(2) Quello che lei ha come Introduzione sarebbe più un'Abstract, che ogni tesi deve avere. Infatti, mi pare che l'abstract dovrebbe essere presente sia in italiano che in inglese. L'abstract dovrebbe essere forse la metà in lunghezza, ma come contenuti è quello che ha scritto lei: un sommario dei contenuti e dei risultati della tesi, incluso un po' di motivazione. L'abstract sarà l'unica cosa che apparirà sulla pagina web della sua tesi, va bene se scritto in stile tecnico, e deve essere ignorato nella tesi stessa (cioè tutto viene detto di nuovo). 

L'introduzione dovrebbe essere più lunga, allargarsi un po' sulla tematica e il background, in questo caso possibilmente anche biologico (un pochino), e di sicuro dovrebbe avere una parte sulle D-strings e le ED-strings, dato che questo è un contributo importante della tesi. Non è però urgente, l'Introduzione la può scrivere quando il resto è finito. 

-->

Come risposta a un commento/domanda sua che ho visto: Si ripete tutto nella parte "Basics" o più avanti, anche se già incluso nell'Introduzione. L'introduzione è pensata per un pubblico generale informatico, mentre la tesi vera inizia con il capitolo 2. 

-->


*** Cose generali, di formattazione etc. 

(a) In generale, le chiedo di includere più citazioni, sia a pagine web/implementazioni (per es. per Rust), che a articoli. 

--> Ok

(b) Come si citano le URL? O come bibitem: @misc{..., ... howpublished = \url{}, ...}, o come footnote. 

--> Ok, grazie

(c) Quando scrive testo in italics, le chiedo di scrivere \textit{...} all'interno dei dollari o degli environment che usano mathmode. (Forse conviene usare i macro.) Per es. all'interno dei environment che usa per gli pseudocodici. Altrimenti il testo viene brutto, perché Latex lo interpreta come singoli caratteri ognuno una variable. 

-->

(d) Per le stringhe, o usiamo stile matematico: A = A_1A_2\cdots A_n, o usiamo stile array: A = A[1..n] = A[1]A[2]\cdots A[n]. Non mischiare i due stile (come per es. nella caption della Fig. 3.2). 

Direi di usare stile array, perché molto più naturale per chi programma e andrà molto bene in questa tesi. 

--> Fatto, se manca in qualche punto me ne accorgerò in riletture future e correggerò

(e) Le chiedo di scrivere $(n_1 \cdot m_2)-1$ invece di $(n1 * m2)-1$ (p.11). Mi ricordo di aver detto nel passato che andava bene anche così, ma ho cambiato idea: la tesi sta diventando sempre più teorica e scrivere $(n1 * m2)$ semplicemente non è lo standard. In particolare, $n1$ è una variable che si scrive così nel codice, ma come variable non si può usarlo così. 

--> Fatto

Questo commento, come anche quelli precendenti, ovviamente si riferisce a tutti i casi simili nella tesi. 

(f) Il linguaggio italiano matematico è molto maschilista. Non possiamo farci tanto, ma se non le dispiace, sarei grata se almeno usasse "nodo genitore" invece di "nodo padre" quando stiamo parlando di un albero. In inglese questo cambiamento è stato fatto vari decenni fa: oggi si dice 'parent, child, sibling node' invece delle versioni vecchie 'father, son, brother node'. Purtroppo in italiano, quanto lo sappia, fin'ora non c'è un alternativa per 'figlio' e 'fratello', ma per 'padre' sì. 

--> Padre sostituito. Posso fare "nodo discendente" e "nodo omologo" o "nodo sullo stesso livello" o "nodo con lo stesso genitore", che dice?

(g) La tesi è in italiano, che va benissimo, come abbiamo già detto. Mi piacerebbe però se potesse mettere i termini originali in inglese al primo uso (in brackets and in italics). Specialmente se il termine italiano è molto diverso (come per "backtracing"). 

-->

(h) complessità di spazio (non: spaziale)

--> Fatto





==================================
========= REGOLE GENERALI ========
==================================

1) Complessità spaziale => complessità di spazio
2) Nodo padre => nodo genitore
3) Per citare gli URL, esempio:
        @misc{rust,
                howpublished = \url,
                key = {https://www.rust-lang.org/},
                title = {Rust language}
        }
4) Per gli array usare sempre notazione A = A[1..n] = A[1]A[2]\cdots A[n]