//! Cet outil scan un repertoire et récupère les métadonnées des fichiers.  
//! Il sauvegarde la structure représentant le scan dans data.json  
//! S pour scan  
//! R pour lire le fichier data.json  
//! Avouter un s à la suite de S ou R pour chercher sur le scan ou le read.
//! Sans arguments, un mode intteractif est lancé avec des questions réponses.  
pub mod cli;
pub mod scan;
pub mod musicfile;
pub mod search;
pub mod markdown;
pub mod mkplaylist;