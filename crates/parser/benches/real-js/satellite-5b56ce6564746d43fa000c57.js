var dataLayer = window.dataLayer || [];
_satellite.getVar("55CookieConsentInit")(); // Execution de la macro qui configure le setup de CC

  CookieConsent.cookielessTracking = true;
  CookieConsent.defaults.approvalScroll = 20;
  CookieConsent.defaults["fr"] = {
    bar: {
      title: "Ce site utilise des cookies.",
      copy: {
        start: "En poursuivant votre navigation, vous acceptez l'utilisation, de la part du Groupe TF1 et de tiers, de cookies et autres traceurs à des fins de mesure d'audience, partage avec les réseaux sociaux, personnalisation des contenus, profilage et publicité ciblée.",
        modify: "",
        end: ""
      },
      mod: "Paramétrer vos choix",
      legal: {
        label: "Politiques cookies"
      },
      sep: "|",
      modify: {
        label: "Modifier",
        icon: "P"
      },
      close: {
        label: "J'ai compris",
        icon: ""
      }
    },
    list: {},
    items: [
      /*{
      bit: 1,
      configurable: 0,
      group: "Fonctionnement",
      name: "Fonctionnement",
      abbr: "F",
      description: "-",
      help: "Le paramétrage ci-dessous vous permet de désactiver les cookies suivant les finalités explicitées. Ces réglages ne seront valides que sur le navigateur que vous utilisez actuellement. En savoir plus."
    },*/
    {
      bit: 2,
      configurable: 1,
      group: "Statistiques",
      name: "Mesure d'audience",
      abbr: "W",
      description: "désactiver cette option nous empêchera de mesurer l'audience, de faire des statistiques et d'améliorer la qualité de nos services.",
      help: "Le paramétrage ci-dessous vous permet de désactiver les cookies suivant les finalités explicitées. Ces réglages ne seront valides que sur le navigateur que vous utilisez actuellement."

    },{
      bit: 4,
      configurable: 1,
      group: "Personnalisation",
      name: "Personnalisation de contenus",
      abbr: "P",
      description: "désactiver cette option nous empêchera de suivre votre navigation et de réaliser des profils, afin de vous faire des recommandations de contenus pour une expérience personnalisée.",
      help: "Le paramétrage ci-dessous vous permet de désactiver les cookies suivant les finalités explicitées. Ces réglages ne seront valides que sur le navigateur que vous utilisez actuellement."

    },{
      bit: 8,
      configurable: 1,
      group: "Réseaux Sociaux",
      name: "Réseaux sociaux",
      abbr: "S",
      description: "désactiver cette option empêchera les réseaux sociaux de suivre votre navigation sur nos services.",
      help: "Le paramétrage ci-dessous vous permet de désactiver les cookies suivant les finalités explicitées. Ces réglages ne seront valides que sur le navigateur que vous utilisez actuellement."
    }, {
      bit: 16,
      configurable: 1,
      group: "Publicité ciblée",
      name: "Publicité ciblée",
      abbr: "A",
      description: "nous diffusons des publicités afin de pouvoir proposer nos services gratuitement à tous les utilisateurs. Les cookies et les profils permettent de vous proposer des publicités plus pertinentes car adaptées à vos centres d'intérêt, et de mesurer l'efficacité des campagnes publicitaires. Désactiver cette option empêchera de suivre votre navigation et de réaliser des profils, afin de vous proposer des publicités adaptées. Vous ne recevrez pas moins de publicités, mais des publicités ne correspondant pas nécessairement à vos centres d'intérêt.",
      help: "Le paramétrage ci-dessous vous permet de désactiver les cookies suivant les finalités explicitées. Ces réglages ne seront valides que sur le navigateur que vous utilisez actuellement."
    }],
    help: {
      abbr: "i",
      text: "En cliquant sur chaque option, vous pouvez contrôler l'activation ou la désactivation du dépôt des cookies et de la création des profils : le bandeau de couleur indique si le dépôt de cookies et la création de profils sont autorisés (vert, sur la gauche) ou refusés (rouge, sur la droite). " +
      "Les cookies techniques (cookies de session , d'authentification et de sécurité) sont indispensables au bon fonctionnement de nos services et ne peuvent être désactivés."
    },
    // configure the Cookie Consent Cookie legal link for "en" webSites
    legalURL: "/cookie/"
};

CookieConsent.defaults["fr"].bar.copy.label = CookieConsent.defaults["fr"].bar.copy.start + CookieConsent.defaults["fr"].bar.copy.modify + CookieConsent.defaults["fr"].bar.copy.end;
if(_satellite.getVar("isMobile") == "true"){
  CookieConsent.defaults.styles.configurable.position = "bottom";
  CookieConsent.defaults["fr"].bar.close.icon = "J'ai compris";
}
cookieconsent('start'); // Lancement du bandeau
