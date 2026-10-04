// Määritellään enum ParcelStatus, joka kuvaa paketin tilaa
// Osa tiloista sisältää nimettyjä kenttiä, kuten kuriiri, vastaanottaja ja palautuksen syy.
// Lisätään #[derive(Clone)], jotta arvot voidaan kloonata tarvittaessa.
#[derive(Debug, Clone)]
enum ParcelStatus {
    Preparing,
    InTransit { courier: String },
    Delivered { recipient: String },
    Returned { reason: String },
}

// funktio, joka palauttaa tilaviestin ParcelStatus-tyypin perusteella
// Käytetään match-lauseketta, joka käsittelee kaikki mahdolliset ParcelStatus-variantit
fn status_message(status: ParcelStatus) -> String {
    match status {
        ParcelStatus::Preparing => "The Parcel is being prepared.".into(),
        ParcelStatus::InTransit { courier } => format!("The parcel is with courier {}.", courier),
        ParcelStatus::Delivered { recipient } => format!("The parcel was delivered to {}.", recipient),
        ParcelStatus::Returned { reason } => format!("The parcel was returned: {}.", reason),
    }
}

// tulostetaan toimitusohje hyödyntämällä if let -rakennetta.
fn print_delivery_note(note: Option<&str>) {
    // jos muuttujassa on arvo (some), poimitaan se ja tulostetaan.
    if let Some(n) = note {
        println!("Delivery Note: {}", n);
    // muuten tulostetaan viesti, että toimitusohjetta ei ole.
    } else {
        println!("There is no delivery note.");
    }
}

// Tarkistetaan if let -rakenteella, onko paketti toimitettu.
// vaikuttamatta itse arvon omistajuuteen.
fn announce_delivery(status: &ParcelStatus) {
    if let ParcelStatus::Delivered { recipient } = status {
        println!("The parcel was delivered to {}.", recipient);
    } else {
        println!("The parcel has not been delivered yet.");
    }
}

// luodaan kuitti let else rakenteella
fn create_receipt(status: ParcelStatus) {
    // yritetään purkaa Delivered-tila. jos se ei täsmää, suoritetaan else lohko (tulostus ja poistuminen).
    let ParcelStatus::Delivered { recipient } = status else {
        println!("Cannot create a receipt: the parcel was not delivered.");
        return;
    };
    // jos tila oli Delivered, tulostetaan kuitti vastaanottajalle.
    println!("Receipt created for {}.", recipient);
}

fn main() {
    // luodaan testiarvot eri tiloille (.into() muuntaa merkkijonoliteraalit String tyypiksi)
    let prep = ParcelStatus::Preparing;
    let transit = ParcelStatus::InTransit { courier: "Aino".into() };
    let delivered = ParcelStatus::Delivered { recipient: "Mikko".into() };
    let returned = ParcelStatus::Returned { reason: "address not found".into() };

    // testataan status_message-funktiota jokaisella variantilla (käytetään .clone(), jotta alkuperäiset säilyvät)
    println!("Status: {}", status_message(prep.clone()));
    println!("Status: {}", status_message(transit.clone()));
    println!("Status: {}", status_message(delivered.clone()));
    println!("Status: {}", status_message(returned.clone()));

    // testataan toimitusohjeita (sekä olemassa oleva, että none)
    print_delivery_note(Some("Leave the parcel at reception."));
    print_delivery_note(None);

    // testataan announce_delivery viittauksella (&)
    announce_delivery(&delivered);
    announce_delivery(&transit);

    // testataan kuitin luontia Delivered-tilalle (tämä kuluttaa lopulta 'delivered'-muuttujan omistajuuden)
    create_receipt(delivered);
    
    // testataan kuitin luontia ei-toimitetuille paketeille.
    create_receipt(ParcelStatus::Preparing);
}