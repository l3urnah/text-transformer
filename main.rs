
slint::include_modules!();
use std::collections::{HashSet, HashMap};
use std::env;
use std::fs::File;
use std::io::{self, Read};

//use slint::SharedString;



#[derive(Clone)]
enum Action {
    Uppercase,
    Double,
    ReplaceWithFoo,
    AppendOriginal,
}

impl Action {
    fn apply(self, mut line: String, pos: usize) -> String {
        match self {
            Action::Uppercase => {
                let mut line_copy = line.clone();
                line_copy.replace_range(pos..pos + 1, &line[pos..pos + 1].to_uppercase());
                line_copy
            }
            Action::Double => {
                let mut line_copy = line.clone();
                line_copy.insert_str(pos, &line[pos..pos + 1]);
                line_copy
            }
            Action::ReplaceWithFoo => {
                format!("line is foo - {}", line)
            }
            Action::AppendOriginal => {
                format!("{} - line is foo", line)
            }
        }
    }
}


struct TextStructure {
    lines: Vec<String>,
}

impl Default for TextStructure {
    fn default() -> Self {
        TextStructure { lines: Vec::new() }
    }
}

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
                position_index += pos + 1;
            }

            if !positions.is_empty() {
                search_results.insert(line_index, positions);
            }
        }

        search_results
    }
}

impl TextStructure {
    fn print_by_line(&self) {
        for (index, line) in self.lines.iter().enumerate() {
            println!("Line {}: {}", index + 1, line);
        }
    }

    fn get_full_text(&self) -> String {
        self.lines.join("\n")
    }

    fn apply_action(&mut self, search_results: &HashMap<usize, Vec<usize>>, action: &Action) {
        for (&line_index, positions) in search_results.iter() {
            let mut modified_line = self.lines[line_index].clone();
    
            for &pos in positions {
                modified_line = action.apply(modified_line, pos);
            }
    
            self.lines[line_index] = modified_line;
        }
    }
}






fn read_and_transform_file(file_path: &str, additional_text: String) -> io::Result<String> {
    // Attempting to open a file
    let result: Result<File, std::io::Error> = File::open(file_path);
    // kann auch als let result: result =  File::open(file_path); 
    // erzeugt werden und hat dann gleich file bzw string eigentlich 
    // siehe use std::{env, result};
    // kann man damit immer noch auf den error arm zugreifen? wsl nicht ^^ 
    // macht dieser result type da einfach unwrap? ^^
    // das error handling hier wird bissl schlecht gemacht, weil glaub ich der error arm nicht gefüllt wird 
    // weil diese func nicht das file zurückgibt aber trotzdem result als rückgabe type hat ^^

    // die kombination aus result type rückgabewert und ? ergibt die form wo die function
    // mit Ok(variable) den type quasi ändert und trotzdem kommt von der func auch der error stream zurück 
    // nur halt im gleichen kanal - true?

    let file:File = result.unwrap();

    // Reading the content of the file into a String
    let mut content: String = String::new();
    let file_size: usize = file.take(usize::MAX as u64).read_to_string(&mut content)?;
    println!("Read {} bytes from file.", file_size);

    // Now you have the file content as a String and can concatenate or process it as needed
    let concatenated_string: String = format!("{} {}", additional_text, content);

    Ok(concatenated_string) // weil ich ein result als returntype hab, kann ich nciht nur den string 
                            // zurückgeben sondern muss den in den ok teil stopfen 
                            // siehe oben ist das hier mit error handling nicht fisch und nicht fleisch 
}

fn _process_result(input: String) -> String {
    // Placeholder for additional processing logic
    input
}

fn process_headings(input: String) -> String{
    // bearbeitet den file string und gibt evtl irgendwas zurück, so wie die header lines für inhaltsverz bzw 
    // aktuell ist das mal ein holzhacker poc und wir konsumieren den string und geben die ganzen combined lines wieder zurück ^^
    // zum prüfen ob die names gut sind und dann kann man die anpassen vorm kopieren oder zumindest kann man das planen 
    // input ist ein file in dem nur eine sequenz an header lines vorkommt 
    // diese wird erfasst und dann umgekehrt 
    // dann werden die entsprechenden lines ersetzt so dass die überschriften (genau) erhalten bleiben
    // es sollte eigentlich am original string der vom file abstammt gearbeitet werden der hier ausgeborgt wird 

    let input_str = input;

    // Split the input string into lines
    let lines: Vec<&str> = input_str.lines().collect();

    // Character to search for
    let search_char = '=';

    // Initialize a vector to store occurrences with line index and count
    let mut occurrences_info: Vec<(usize, usize)> = Vec::new();

    // Iterate over lines and analyze for occurrences
    for (line_index, line) in lines.iter().enumerate() {
        let count = line.chars().filter(|&c| c == search_char).count();

        // If there are more than one occurrences, store line index and count
        if count > 1 {
            occurrences_info.push((line_index, count));
        }
    }
    // find lowest and highest number of header markers 

       // Sample input: Vec<(usize, usize)>  occurrences_info
    //    let vec_of_tuples = vec![
    //     (31, 4), (32, 8), (34, 8), (36, 8), 
    //     (37, 10), (38, 12), (39, 16), 
    //     (40, 4), (41, 6), (43, 4)
    // ];

    // Extract distinct values from the second field and sort them in descending order
    let distinct_values: Vec<usize> = occurrences_info.iter().map(|&(_, occurrence)| occurrence).collect();
    let mut sorted_values: Vec<usize> = distinct_values.clone();
    sorted_values.sort_by(|a, b| b.cmp(a));
    println!("sorted values: {:?}", sorted_values);

    // zuletzt deswegen, weil wie die sorterei gar nicht brauchen, aber auch weil wir distinct values als "helper" nutzen um auf das zweite feld im tupel zugreifen zu können
    // sollte das erzeugen von hashset direkt vom occurences info vector her passieren 
    // dann hinweis wenn es mehr als fünf sind und bingo -> keep line indexes and just replace the parts with equal sign 
    // ist ein table mit zu ändernden line indices und der gewünschten aktion eine gute idee? 
    // das übergibt man dann einer durchführungs func die die einzelnen transformer implementiert ^^

    // Create a HashSet from distinct_values for counting actual unique values
    let unique_set: HashSet<_> = distinct_values.iter().cloned().collect();

    // Print the actual number of unique values
    println!("Number of unique values: {}", unique_set.len());
    println!("unique values: {:?}", unique_set);

    // Check if there are more than five unique values -- todo: aus genau diesem unique set erzeugen wird das mapping für die neuen werte 
    // die neuen werte sind immer 6 bis 2 und was auch immer davor war es wird dann so -> siehe unten ist dann alle obsolet 
    if unique_set.len() > 5 {
        println!("Warning: More than five unique values found in sorted_values"); // do we want to put this into the text? it will be part of the wiki page then...
    }
    // todo: wenn es wirklich mehr als 5 werte gibt, dann diese immer an max value mappen - ist gegeben weil alles was übersteigt ist eh zu groß und wird dann einfach so angezeigt 
    // todo: mapping so aufbauen, dass es nur die fünf indizes gibt und werte aus unique_set dort zugeordnet werden -> baut sich selbst nach der anzahl der uniquen werte auf 
    // todo: das finale mapping anpassen weil wir ja die alten werte suchen und diese nicht immer finden und dafür nicht 0, 0 zurückbekommen wollen -> siehe unten trick 17
    // das gilt aber nur für mehr als 5 unique werte - der normale fall sollte einfach funken - ois funkt 



    // Create a mapping from original values to normalized weights - wir erzeugen tuples aus den werten von occinfo und sorted (sollte umsortiert heissen oder sowas)
    // dann mappen wir statt der ursprünglichen werte die neuen werte auf die gleichen stellen 
    // somit ist es umgedreht und nicht mehr genau gleich -> ziel war es, unabhängig davon, wie genau die überschriften struktur was sequenz und start und ende betrifft 
    // neue überschrift marker zu erzeugen die genau dem entsprechen was dokuwiki mag -> 5 ebenen max auf einanderfolgend start mit 1 = 
    // das ist noch nciht ganz gegeben aber mal close enough -> den höchsten wert kann ich ja abfangen und die anderen sind meh
    // mal schauen wie oft das nicht stimmt, weil supergut ausschauen tut das nicht -> todo: was ist genau die gefahr wenn ich das ohne vewränderung mache also die values einfach umkehre?
    // aus 2 wird 6 usw -> das kann ich genau so mit mapping und relation tuple vector machen 
    // ohne plus eins ist die höchste ebene vielleicht nur 1 und das mag man nicht - die höchste ebene soll schon zwei sein -> check verhalten - todo: 
    // das plus eins ist nur damits keine 0 gibt, weil der index vom enumerate fängt halt bei null an... 
    // wozu der default wert nötig ist? es kann ja nicht sein, dass es keinen eintrag gibt, oder? todo: und was soll ein 0 0 eintrag schon können..
    // was passiert wirklich, wenn die sequenz nicht linear steigt? 
    // mehr als fünf = verarbeitet dokuwiki nciht als heading -> verhalten?

    let mapping: Vec<(usize, usize)> = unique_set
        .iter()
        .enumerate()
        .map(|(index, &value)| (value, index + 1)) // Adding 1 since index starts with zero 
        .collect();
    println!("mapping values: {:?}", mapping);
    // Transform the original vector using the mapping
    let transformed_vec: Vec<(usize, usize)> = occurrences_info
        .iter()
        .map(|&(index, occurrence)| (index, mapping.iter().find(|&&(value, _)| value == occurrence).unwrap_or(&(0, 0)).1))
        .collect();
    // dieser trick mit unwrapor dem default und dann dem .1 davon ist scheinbar der bessere weg, als an das find noch mal ein map anzuhängen und damit auf den 2 teil vom tuple zuzugreifen 
    // Print the original and transformed vectors
    println!("Original vector: \n{:?}\n", occurrences_info);
    println!("Transformed vector: \n{:?}", transformed_vec);


    // mediawiki
    // heading is determined by the number of = -> higher is higher - highest is six 
    
    // dokuwiki 
    // heading is determined by the number of = -> highest is five! which is the first level and goes down from there 
    // so we need to know the highest number first, to be able to construct a nested structure 
    
    // Original vector:
    // [(31, 4), (32, 8), (34, 8), (36, 8), (37, 10), (38, 12), (39, 16), (40, 4), (41, 6), (43, 4)]
    
    // Transformed vector:
    // [(31, 8), (32, 4), (34, 4), (36, 4), (37, 3), (38, 2), (39, 1), (40, 8), (41, 7), (43, 8)]

    // 
    // Print results with lines that have occurrences
    for (line_index, count) in occurrences_info.iter() {
        let line = lines[*line_index];

        println!("Line {}: Occurrences count: {}\n{}", line_index, count, line);
        //println!("Line Content: {}", line);
        //println!("------------");




    }
    //let return_string: String = String::from("bla");
    //return return_string;
    //let merged_lines: String = lines.join("\n");
    //return merged_lines;
    return lines.join("\n");
}



fn main() -> Result<(), slint::PlatformError> {
    println!("Current working directory: {:?}", env::current_dir());



    let ui = AppWindow::new()?;
    let ui_handle = ui.as_weak(); 
    ui.on_action(move |bla: slint::SharedString | {
        let ui: AppWindow = ui_handle.unwrap();
        let num: f64 = match bla.trim().parse::<f64>() {
            Ok(n) => n,
            Err(e) => {
              let err_text: String = e.to_string();
              println!("Failed to parse number: {}", err_text);
              return;   // damit wird nicht der letzte wert returned, was zu einem type error führt, 
                        //weil num ja kein string sein kann 
            }
          };
        let num1: f64 = num * 0.3;
        //let result: String = format!("Bla: {:.2}\n", num1);
        // memory wise ist es wsl besser man übergibt die var gleich an eine func und sie wird dort 
        // konsumiert, wenn wir eh nur einen rückgabewert behalten wollen - oder wir modifizieren die var
        // konret haben wir hier aber messi anxiety und wollen num behalten oder so wsl...
        
        // Define the file path and additional text// calling the file load function
        let file_path: &str = "data\\file-01.txt";
        // todo: get filepath from appwindow
        let additional_text: String = String::from("---Beginn File---\n");
        // im ggs zur line davor wo eine "globale" referenz erzeugt wurde die man weiter benutzen kann
        // wurde für additional_text ein string slice erzeugt der von der nächsten func konsumiert wird
        // in dem man eine load func von string nutzt - einfach = "bla" schreiben geht nicht
        // spart man sich in rust irgendwie allg. getter und setter bauen? 
        let astring: String = format!("Bla: {:.2}\n{}", num1, additional_text);
        // Call the function with the variables
        let file_result: Result<String, io::Error> = read_and_transform_file(file_path, astring);
        let file_content: String = process_headings(file_result.unwrap());


        println!("bla: {} {}",num1, additional_text);
        println!("{}",additional_text); 
        // ownen wir noch weil nur für format! genutzt -> macro - hat mit macro nichts zu tun sondern wir sind im main scope? 
        // beides falsch :) -> :ident vs :expr leads to identifier oder value 

        // println!("{}",astring); // failed, ja 
       
        //let mut text_structure = TextStructure::new(file_content);

        
          
        //println!("Original Text:");
        //text_structure.print_by_line();
  
        

        // let text = String::from("This is a sample text with some words. Another foo line.");
        let mut text_structure = TextStructure::default();
        text_structure.lines = file_content.lines().map(String::from).collect();
    
        println!("Original Text:");
        text_structure.print_by_line();
    
        let searcher = Searcher::new(&text_structure.lines);
    
        let search_strings = vec!["sample", "words", "foo"];
        let actions = vec![
            Action::Uppercase,
            Action::Double,
            Action::ReplaceWithFoo,
            Action::AppendOriginal,
        ];
    
        for search_string in search_strings {
            let search_results = searcher.search_string(search_string);
            for action in &actions {
                text_structure.apply_action(&search_results, action);
            }
        }
    

        println!("\nModified Text:");
        println!("{}", text_structure.get_full_text());
    //;
    //file_content.into()
        let handover_string: String = text_structure.get_full_text();

        ui.set_results(handover_string.into());
    });
  
    ui.run()
}