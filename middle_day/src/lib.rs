extern crate chrono;

pub use chrono::{Datelike, NaiveDate, Weekday as wd};

pub fn middle_day(year: i32) -> Option<wd> {
    /*
        NaiveDate::from_ymd_opt(year, 12, 31) : Tente de créer une date pour
        le 31 décembre de l'année spécifiée. La méthode from_ymd_opt retourne
        une Option<NaiveDate>, qui est Some si la date est valide et None sinon
        Le point d'interrogation ? déstructure l'Option, retournant None si la
        date est invalide et continuant l'exécution si elle est valide.
    */
    // Cree une date pour le 31 12 de l'annee year
    let lasday = NaiveDate::from_ymd_opt(year, 12, 31)?;
    /*
        lastday.ordinal() : Appelle la méthode ordinal sur last_day pour obtenir
        le numéro ordinal du jour dans l'année (par exemple, 31 décembre est le 365e
        jour de l'année pour une année non bissextile).
    */
    let day_in_year = lasday.ordinal();

    /*
        if days_in_year % 2 != 0 : Vérifie si le nombre de jours dans l'année est impair.
        Si le nombre de jours est impair :
        NaiveDate::from_ymd_opt(year, 1, 1)? : Crée une date pour le 1er janvier de l'année spécifiée.
        checked_add_signed(chrono::Duration::days(days_in_year as i64 / 2)) : Ajoute la moitié du nombre
        total de jours de l'année (troncature entière) au 1er janvier pour trouver le jour central.
        .map(|d| d.weekday()) : Si l'addition réussit, transforme la date résultante en son jour de la
        semaine (Weekday).
        Si le nombre de jours est pair, la fonction retourne None.
    */
    if day_in_year % 2 != 0 {
        NaiveDate::from_ymd_opt(year, 1, 1)?
            .checked_add_signed(chrono::Duration::days(day_in_year as i64 / 2))
            .map(|d| d.weekday())
    } else {
        None
    }
}