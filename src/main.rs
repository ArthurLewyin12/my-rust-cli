use colored::*;
use dialoguer::{theme::ColorfulTheme, Input};
use indicatif::{ProgressBar, ProgressStyle};
use inquire::{Select, Confirm};
use rand::{thread_rng, Rng};
use rand::distributions::Alphanumeric;
use std::thread;
use std::time::Duration;

fn main() {
    afficher_banniere();

    let choix = Select::new(
        "Que voulez-vous générer ?",
        vec!["Mot de passe", "Token"],
    )
        .prompt()
        .expect("Erreur lors de la sélection.");

    match choix {
        "Mot de passe" => generer_mot_de_passe(),
        "Token" => generer_token(),
        _ => println!("Option inconnue."),
    }
}

fn afficher_banniere() {
    println!("{}", "\nGÉNÉRATEUR DE SÉCURITÉ".bright_cyan().bold());
    println!("{}", "===================".cyan());
    println!("{}", "Version 2.0".bright_blue());
    println!("{}\n", "Développé avec ❤️  par Irie Emmanuel".bright_black());
}

fn simuler_generation(message: &str) {
    let pb = ProgressBar::new(100);
    pb.set_style(ProgressStyle::default_bar()
        .template("{spinner:.green} [{bar:40.cyan/blue}] {pos}% {msg}")
        .unwrap()
        .progress_chars("#>-"));

    for _i in 0..100 {
        pb.set_message(message.to_string());
        pb.inc(1);
        thread::sleep(Duration::from_millis(20));
    }
    pb.finish_with_message("Terminé!");
    thread::sleep(Duration::from_millis(500));
}

fn generer_mot_de_passe() {
    println!("\n{}", "Configuration du mot de passe".yellow());
    println!("{}", "==========================".yellow());

    // Correction de l'entrée utilisateur
    let longueur: usize = Input::<String>::with_theme(&ColorfulTheme::default())
        .with_prompt("Quelle longueur pour le mot de passe ?")
        .default("16".into())
        .interact()
        .expect("Erreur lors de la saisie")
        .parse()
        .expect("Veuillez entrer un nombre valide");

    let inclure_symboles = Confirm::new("Inclure des symboles (@, #, !) ?")
        .with_default(true)
        .prompt()
        .expect("Erreur lors de la saisie.");

    let inclure_chiffres = Confirm::new("Inclure des chiffres (1, 2, 3) ?")
        .with_default(true)
        .prompt()
        .expect("Erreur lors de la saisie.");

    let inclure_majuscules = Confirm::new("Inclure des majuscules (A, B, C) ?")
        .with_default(true)
        .prompt()
        .expect("Erreur lors de la saisie.");

    simuler_generation("Génération du mot de passe sécurisé");

    let mot_de_passe = creer_mot_de_passe(longueur, inclure_symboles, inclure_chiffres, inclure_majuscules);

    println!("\n{}", "🔐 Résultat :".green().bold());
    println!("{} {}", "►".bright_green(), mot_de_passe.white().on_bright_black().bold());

    afficher_evaluation_securite(&mot_de_passe);

    let sauvegarder = Confirm::new("Voulez-vous sauvegarder ce mot de passe ?")
        .with_default(false)
        .prompt()
        .expect("Erreur lors de la saisie.");

    if sauvegarder {
        sauvegarder_mot_de_passe(&mot_de_passe);
    }
}

fn generer_token() {
    println!("\n{}", "Configuration du token".yellow());
    println!("{}", "====================".yellow());

    // Correction de l'entrée utilisateur pour le token aussi
    let longueur: usize = Input::<String>::with_theme(&ColorfulTheme::default())
        .with_prompt("Quelle longueur pour le token ?")
        .default("32".into())
        .interact()
        .expect("Erreur lors de la saisie")
        .parse()
        .expect("Veuillez entrer un nombre valide");

    simuler_generation("Génération du token sécurisé");

    let token: String = thread_rng()
        .sample_iter(&Alphanumeric)
        .take(longueur)
        .map(char::from)
        .collect();

    println!("\n{}", "🔑 Votre token :".green().bold());
    println!("{} {}", "►".bright_green(), token.white().on_bright_black().bold());

    let sauvegarder = Confirm::new("Voulez-vous sauvegarder ce token ?")
        .with_default(false)
        .prompt()
        .expect("Erreur lors de la saisie.");

    if sauvegarder {
        sauvegarder_mot_de_passe(&token);
    }
}

fn creer_mot_de_passe(
    longueur: usize,
    inclure_symboles: bool,
    inclure_chiffres: bool,
    inclure_majuscules: bool,
) -> String {
    let mut charset = "abcdefghijklmnopqrstuvwxyz".to_string();

    if inclure_symboles {
        charset.push_str("!@#$%^&*()-_=+[]{}|;:',.<>?/`~");
    }
    if inclure_chiffres {
        charset.push_str("0123456789");
    }
    if inclure_majuscules {
        charset.push_str("ABCDEFGHIJKLMNOPQRSTUVWXYZ");
    }

    let mot_de_passe: String = thread_rng()
        .sample_iter(rand::distributions::Uniform::new_inclusive(0, charset.len() - 1))
        .take(longueur)
        .map(|i| charset.chars().nth(i).unwrap())
        .collect();

    mot_de_passe
}

fn afficher_evaluation_securite(mot_de_passe: &str) {
    println!("\n{}", "📊 Évaluation de la sécurité :".cyan().bold());

    let longueur = mot_de_passe.len();
    let contient_majuscules = mot_de_passe.chars().any(|c| c.is_uppercase());
    let contient_chiffres = mot_de_passe.chars().any(|c| c.is_numeric());
    let contient_symboles = mot_de_passe.chars().any(|c| !c.is_alphanumeric());

    let mut score = 0;
    if longueur >= 12 { score += 1; }
    if contient_majuscules { score += 1; }
    if contient_chiffres { score += 1; }
    if contient_symboles { score += 1; }

    let pb = ProgressBar::new(4);
    pb.set_style(ProgressStyle::default_bar()
        .template("[{bar:40.green/red}] {pos}/{len}")
        .unwrap()
        .progress_chars("█░░"));

    pb.set_position(score);

    println!("\nForce du mot de passe : {}", match score {
        4 => "Excellent !".bright_green().bold(),
        3 => "Très bon".green(),
        2 => "Moyen".yellow(),
        1 => "Faible".red(),
        _ => "Très faible".bright_red(),
    });

    println!("\nDétails :");
    println!("► Longueur : {}", if longueur >= 12 { "✓".green() } else { "✗".red() });
    println!("► Majuscules : {}", if contient_majuscules { "✓".green() } else { "✗".red() });
    println!("► Chiffres : {}", if contient_chiffres { "✓".green() } else { "✗".red() });
    println!("► Symboles : {}", if contient_symboles { "✓".green() } else { "✗".red() });
}

fn sauvegarder_mot_de_passe(contenu: &str) {
    let chemin = Input::<String>::with_theme(&ColorfulTheme::default())
        .with_prompt("Entrez le nom du fichier")
        .default("passwords.txt".into())
        .interact()
        .expect("Erreur lors de la saisie");

    let pb = ProgressBar::new_spinner();
    pb.set_message("Sauvegarde en cours...");
    pb.enable_steady_tick(Duration::from_millis(100));

    use std::fs::OpenOptions;
    use std::io::Write;

    let mut fichier = OpenOptions::new()
        .create(true)
        .append(true)
        .open(&chemin)
        .expect("Erreur lors de l'ouverture du fichier.");

    writeln!(fichier, "{}", contenu).expect("Erreur lors de la sauvegarde.");

    thread::sleep(Duration::from_secs(1));
    pb.finish_with_message(format!("✅ Sauvegardé dans '{}'", chemin).green().to_string());
}