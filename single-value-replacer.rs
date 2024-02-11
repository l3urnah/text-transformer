use std::collections::HashMap;
use std::fs;

// das makro reduziert jede position um eins mehr -> (ist gleich index) - modifiy for different values	
// gilt für erstzen von 3 mit 2 
macro_rules! transform_search_vector_for_noteq_index {
    ($vector:expr) => {{
        $vector.iter_mut()
            .enumerate()
            .for_each(|(index, element)| {
                if index != 0 {
                    *element -= index;
                }
            });
    }};
}

macro_rules! apply_action {
    (ReplaceNumOfSingleQuotes, $line:expr, $positions:expr, $num:expr) => {{
        let mut modified_line = $line.to_owned();
        assert!($num == 2 || $num == 3, "Invalid value for num in ReplaceNumOfSingleQuotes");
        let replacement = if $num == 2 { "**" } else { "//" };
        
        for &pos in $positions.iter() {
            println!("pos: {}", pos);
            if modified_line.is_char_boundary(pos) && modified_line.is_char_boundary(pos + $num) {
                modified_line.replace_range(pos..pos + $num, replacement);
            } else {
                // Handle invalid indices gracefully
                println!("Invalid indices for replacement.");
            }
        }
        modified_line
    }};
    // Add more patterns for other Action variants as needed
    (_default, $line:expr, $pos:expr, $num:expr) => {{
        // For other actions, just return the original line
        $line.to_owned()
    }};

}


#[derive(Clone)]
enum Action {
    ReplaceTwoSingleQuotes,
    ReplaceThreeSingleQuotes,
    ReplaceFiveSingleQuotesForCursItal,
}


// we actually need a num modifier because the number of single quotes
        // determines the range used to replace the single quotes
        // oder ist es auch hier besser mit string replace zu löschen und 
        // in nächsten schritt einzusetzen? --> aktuell siehe macro apply_action 

        // hier ist der design "fehler" der verhindert, dass der vorschlag von unten bei apply_action 
        // einfach funktioniert -> action erzeugt eine immutable reference / slice und arbeitet daran 
        // deswegen muss man nicht nur search_results neu initialisieren sondern eben auch die struktur ändern
        // wenn man vom "funktionieren" weg will -> action sollte nicht selbst die referenz sein sondern an 
        // mutable quelle arbeiten und unabhängig bleiben 
        // oder die action sache bleibt auch der besitzer vom text und wird so behandelt 
        // man könnte auch sagen elegant die sache gelöst, weil die action ist die, die verändert 
        // und apply_action bleibt unabhängig und bietet das mutable an und änderts selbst 
        // der große text bleibt ein slice und ein einziges mut 
        // wsl ergibt sich auch genau daraus der need für den negativen index, den sollte man eigentlich nicht brauchen
        // wenn dann einen positiven wenn mit weniger erstetzt wird als da war 

        // korrigieren ist immer nötig, weil positions mit der länge vom gefundenen string addiert werden
        // > searcher arbeitet immer auf einer borrowd ref und macht somit keine mem aktive copy 
        // in main wird genau an der mutierbaren kopie von der originalen text_struct gearbeitet 
        // das kann man ändern wenn man will. die search basis muss immer initialisiert werden, so oder so..
        // kaun ned imma ois mutable sain
impl Action {
    fn apply(&self, line: &str, positions: Vec<usize>) -> String {
        match self {
            Action::ReplaceThreeSingleQuotes => apply_action!(ReplaceNumOfSingleQuotes, line, positions, 3),
            // Add more patterns for other Action variants as needed
            Action::ReplaceTwoSingleQuotes => apply_action!(ReplaceNumOfSingleQuotes, line, positions, 2),

            Action::ReplaceFiveSingleQuotesForCursItal => {
                // in warheit kann man genau das nehmen und daraus eine fn machen die in main auf einen mutable text angewendet wird.
                let mut modified_line: String = line.to_owned();
                let local_search_string: &str = "'''''";
                // Remove the search_string from the line just once
                modified_line = modified_line.replace(local_search_string, "");
                for &pos in positions.iter() {
                    if pos % 2 == 0 {modified_line.insert_str(pos, "//**");}
                    else {modified_line.insert_str(pos, "**//");}

                    // hoffentlich ist das kein issue, dass wir jetzt abhängig davon ob 
                    // das vorkommen an einer geraden oder ungeraden position ist, 
                    // mal **// //** und mal umgekehrt ersetzen i.e. //** **// 
                }
                modified_line
            }

            _ => apply_action!(_default, line, pos, 0),

        }
    }
}

// fn insert_double_slash(line: &mut String, positions: Vec<usize>) {
//     for (index, &position) in positions.iter().enumerate() {
        
//         if index % 2 == 0 {
//             // Execute command for even indices 
//             println!("Posfirst {}", position);
//             line.insert_str(position, "//**");
//         } else {
//             println!("Possecond {}", position);
//             line.insert_str(position, "**//");
//         }
//     }
// }


struct Searcher<'a> {
    lines: &'a [String],
}

impl<'a> Searcher<'a> {
    fn new(lines: &'a [String]) -> Self {
        Searcher { lines }
    }

    fn search_string(&self, search: &str) -> HashMap<usize, Vec<usize>> {
        let mut search_results = HashMap::new();

        for (line_index, line) in self.lines.iter().enumerate() {
            let mut positions = Vec::new();
            let mut position_index = 0;

            while let Some(pos) = line[position_index..].find(search) {
                positions.push(pos + position_index);
                position_index += pos + search.len();
            }

            if !positions.is_empty() {
                search_results.insert(line_index, positions);
            }
        }

        search_results
    }
}

fn apply_action(lines: &mut Vec<String>, search_results: &HashMap<usize, Vec<usize>>, action: &Action) {
    for (&line_index, positions) in search_results.iter() {
        if let Some(original_line) = lines.get_mut(line_index) {
            let mut modified_line = original_line.clone();

            modified_line = action.apply(&modified_line, positions.clone());
            
            *original_line = modified_line;
        }
    }
}
// das ziel hier sollte sein, direkt an lines zu arbeiten ohne diese zu kopieren
// und die zeile modified_line = action.apply(&modified_line, positions.clone());
// zu sowas wie action.apply(modified_line, &positions); zu machen -> siehe doc file
// ausserdem kann man search results eigentlich übergeben, dann hab ich die da und sind danach weg 
// für mehrere actions auf dem gleichen searcher allerdings dann wieder schlecht.. 

// bin sehr sicher noch immer unter 20 stunden 


fn main() {
    let content = String::from("This is a ''sample'' text with some '''words'''. Another '''''foo''''' line. Another '''''foo''''' line. Another '''''foo''''' line.\n'''''foo'''''\nbla blubb '''''foo'''''.
    fasdfa
    '''''ultrafettkursiv'''''
    ''kursiv'', ''kursiv'' '''words''', ''kursiv'', ''kursiv'' - '''words''', ''kursiv'', ''kursiv'' - '''words'''
    '''words'''
    ");
    
    // let path = "data\\file-01.txt";

    // let content = fs::read_to_string(path)
    //     .expect("Unable to read file");

    // this is some kind of type flexible text string genereator <Vec<_>> creates a vec with elements that can have any type _
    let mut text_structure: Vec<String> = content.lines().map(String::from).collect::<Vec<_>>();

    println!("Original Text:");
    for (index, line) in text_structure.iter().enumerate() {
        println!("Line {}: {}", index + 1, line);
    }


    ///// search five single quotes for cursive bold

    let searcher: Searcher<'_> = Searcher::new(&text_structure);
    let search_string: &str = "'''''";
    let actions: Vec<Action> = vec![Action::ReplaceFiveSingleQuotesForCursItal];

    let mut search_results: HashMap<usize, Vec<usize>> = searcher.search_string(search_string);
    println!("search results five: \n{:?}",search_results);
    //search_results[&1] = search_results[&1]-1; // this does not work...

    // when replacing a string with some shorter string we need to adjust 
    // the second occurance index by the number of different characters of one replacement / occurance
    // need to do that for each new searcher 

    // hier haben wir so umgebaut, dass alle zweiten elemente transformiert werden, um mehrere vorkommen
    // zu behandeln -> da aber mit jedem vorkommen auch der gesamt index für weitere vorkommen verändert wird
    // da ja die doppelte anzahl von zeichen fehlt -> wird dann für jedes vorkommen mehr  
    // for (_, vector) in search_results.iter_mut() {
    //     for (index, second_element) in vector.iter_mut().enumerate() {
    //         if index % 2 == 1 {
    //             *second_element -= 1;
    //         }
    //     }
    // }

    // search_results.iter_mut().for_each(|(_, vector)| {
    //     *vector = vector.iter_mut()
    //         .enumerate()
    //         .map(|(index, element)| {
    //             match index {
    //                 1 => *element - 1,
    //                 2 => *element - 2,
    //                 3 => *element - 3,
    //                 4 => *element - 4,
    //                 5 => *element - 5,
    //                 _ => *element,
    //             }
    //         })
    //         .collect();
    // });
    
    search_results.iter_mut().for_each(|(_, vector)| {
        transform_search_vector_for_noteq_index!(vector);
    });

    // nice, das gilt für alle multiplen vorkommen -> todo: herausheben und für alle actions nutzen 
    // seit wir den search vector nicht mehr im apply iterieren müssten wir doch eigentlich 
    // jetzt auch diese transformation auch von dort aus ausführen können?
    // # declutter and reduce main function - gutes beispiel dafür wo man das aber nicht machen sollte 
    // im ggs zu einem makro für den zweiten teil von einer searcher definition btw 
    // die search results angreifen irgendwo anders als hier in main ist ein grosses nono 
    // wenn man davon ausgeht, dass das eine koppelung bedeutet und man nicht mehr wüsste das sowas 
    // ausgeführt werden kann und daraus unerwartete dinge entstehen 
    // was ich mich aktuell frage ist, let searcher öfter als einmal ausführen ist scheinbar ok
    // und führt auch scheinbar dazu, dass eine neue instanz von searcher entsteht 
    // da wir aber jetzt immer die referenz zu modified text nutzen können wir ja eigentlich einfach wieder 
    // searcher.search_string(search_string) ausführen und dann die search results nutzen
    // todo: testen ob wirklich alle versionen von viele actions für ein search_result oder nur eine für viele results alles gut geht 
    // da im searcher auch searchresults lebt und dort appended wird, passiert ohne neu initialisieren schmarrn
    // -> reinit function for searcher -> dann kann man sich das sparen - bzw eigtl eh ein eindeutiges muss
    // also todo: search_string muss sich selbst die var refreshen -> macht null sinn, dass man das aufruft 
    // und an die ergebnise vom letzten lauf anhängt - ursache ist eine andere -> invalid indices for replacement
    // den searcher nicht jedes mal neu erzeugen ist eine schöne idee, aber da wir ihn nicht updaten bleibt er scheinbar 
    // unverändert und es wirkt aktuell gar nicht so, dass wir immer am letzten stand von modified_text arbeiten
    // so kommen die falschen indizes zustande -> und dann trifft man die halben character positions -> fehler invalid indices 
    // searcher neu machen mit der aktuellen version von modified_text, so wie bisher ist also gut 
    // ausser man überlegt sich noch mal genau wann wir da mit welcher version von modified_text arbeiten
    // i.e. apply_action vs die version die im searcher gespeichert ist
    // die idee den searcher immer einfach auf der mutierten referenz weiterarbeiten zu lassen ist schön,
    // funkt aber so nicht - und ist auch bissl unnedig... 
    

    println!("search results five after: \n{:?}",search_results);
    //let mut modified_text = text_structure.clone();

    for action in &actions {
        apply_action(&mut text_structure, &search_results, action);
    }

    // println!("\nModified Text after five single quotes for cursive italic:");
    // for (index, line) in modified_text.iter().enumerate() {
    //     println!("Line {}: {}", index + 1, line);
    // }


    ///// search three single quotes on the text without five single quotes in a row 
    //// so we find three single quotes and then two at last - wir haben das problem hier aber gar nicht 
    //// da keine teil strings gefunden werden anscheinend
    let searcher: Searcher<'_> = Searcher::new(&text_structure);
    let search_string: &str = "'''";
    let actions: Vec<Action> = vec![Action::ReplaceThreeSingleQuotes];

    let mut search_results: HashMap<usize, Vec<usize>> = searcher.search_string(search_string);
    println!("search results: \n{:?}",search_results);
    //search_results[&1] = search_results[&1]-1; // this does not work...

    // when replacing a string with some shorter string we need to adjust 
    // the second occurance index by the number of different characters of one replacement / occurance
    // need to do that for each new searcher 

    search_results.iter_mut().for_each(|(_, vector)| {
        transform_search_vector_for_noteq_index!(vector);
    });

    println!("search results2: \n{:?}",search_results);
    //let mut modified_text_0: Vec<String> = modified_text.clone();

    for action in &actions {
        apply_action(&mut text_structure, &search_results, action);
    }


    ///// search two single quotes 
    
    // define search string and action(s)
    let search_string: &str = "''";
    let actions: Vec<Action> = vec![Action::ReplaceTwoSingleQuotes];

    // todo: make a macro from the following lines 
    // create a new searcher with the current version of modified text
    let searcher: Searcher<'_> = Searcher::new(&text_structure);
    let search_results: HashMap<usize, Vec<usize>> = searcher.search_string(search_string);

    for action in &actions {
        apply_action(&mut text_structure, &search_results, action);
    }
    drop(search_results); // explicitly drop here - after that we can just use 
    // let search_results = searcher.search_string(search_string); for the next search 

    // if search_results is only created once as mutable we do not have to change mutable or not for 
    // searches that do not replace different numbers of characters

    // and also reuse the actual var instead of creating a new one with the same name (shadow)
    // therefore avoiding that instances of search_resulst hang around in main and use space
    // for really large docs with lots of search results
    // actual todo: optional: make a macro for that and create a function for every search 
    // and let scope handle lifetimes -> siehe C:\Users\burns\Documents\python-stuff\loadandtransform\docs\talking-about-the-loops-and-ownerships.md
        
    println!("\nModified Text:");
    for (index, line) in text_structure.iter().enumerate() {
        println!("Line {}: {}", index + 1, line);
    }
}
