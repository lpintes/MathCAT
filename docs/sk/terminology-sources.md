# Slovak spoken-math terminology: sources

Research done 2026-11-10 for the Slovak (sk) MathCAT speech rules.

## Summary

No Slovak document was found that defines how to read whole mathematical
expressions aloud for blind users. No Slovak equivalent of ClearSpeak or
MathSpeak turned up, and no reading rules from the Levoča library or the
schools for the blind. What exists:

- authoritative Slovak names for individual symbols (the 2021 Slovak Braille
  rules from SABP and the NVDA Slovak symbol dictionary);
- the Slovak localization of the Lambda editor: a 2009 description of its
  spoken output for structures (fraction start, fraction bar, end of
  fraction), and the spoken strings themselves in an installed 2015 version
  (fractions, powers, relations, sets, sums, limits, integrals, derivatives);
- school and university materials that state readings for powers, roots,
  fractions, derivatives and definite integrals.

Spoken readings for limits, sums, absolute value, functions and indices
(subscripts) have no explicit Slovak source apart from Lambda. Entries with
no source at all are marked as unsourced suggestions.

## 1. Sources

- SABP 2021, Braille rules. "Pravidlá písania a používania Braillovho písma
  v Slovenskej republike" (rules for writing and using Braille in the Slovak
  Republic). Published by Slovenská autorita pre Braillovo písmo (SABP), the
  Slovak Braille Authority at Slovenská knižnica pre nevidiacich Mateja
  Hrebendu (SKN), Levoča, 2021, ISBN 978-80-18-10144-5.
  URL: http://liblouis.io/braille-specs/slovak/pravidla-bp01.pdf
  - What it covers: Braille codes with a "slovný popis" (verbal description)
    column for each symbol. That includes basic operators and relations
    (section 13.2), Greek letters, percent, degree, prime, brackets and units.
    It also has spoken examples of numbers and formulas ("pí krát r na druhú",
    "tri celých štrnásť stotín").
  - Trust: high. It is the official national standard body for Braille, and
    its descriptions name symbols as Slovak blind readers learn them. It only
    covers "basic technical notation"; a fuller math part is announced but
    was not found online.
- NVDA Slovak symbol dictionary, `source/locale/sk/symbols.dic` in
  nvaccess/nvda. Checked in the local clone
  `C:\vcs\github.com\nvaccess\nvda`, last commit to the file 84d079ccf
  (2025-11-25).
  URL: https://github.com/nvaccess/nvda/blob/master/source/locale/sk/symbols.dic
  - What it covers: Slovak names for about 150 math symbols (relations, set
    operators, logic, integrals, roots, vulgar fractions, number sets,
    arrows).
  - Trust: medium. Slovak users of the most common screen reader hear these
    names every day, so they are familiar. But they come from community
    translation, some are literal translations of Unicode names, and a few are
    wrong. For example, ℍ is given as "zlomky" (fractions), 𝕁 as
    "kladné (celé) čísla", and "≈" as "takmer úplne sa rovná".
- Horňanský 2009, student thesis on the Lambda editor. Martin Horňanský,
  "Sprístupňovanie matematiky nevidiacim na Slovensku prostredníctvom editora
  Lambda" (making math accessible to blind people in Slovakia with the Lambda
  editor). Študentská vedecká konferencia (student research conference)
  2009, FMFI UK Bratislava; supervisor I. Kohanová. It documents the Slovak
  localization of the Lambda editor, made at the UK Support Centre (Podporné
  centrum) with Peter Lecký.
  URL: https://www.duxburysystems.org/codes/slovak_math.pdf
  - What it covers: spoken output of Lambda for compound fractions (table
    4.2), set operator names, the notion of "koniec odmocniny" (end of root),
    and a Slovak rendering of the Aster/MathTalk grouping example.
  - Trust: medium. It is a student work, but it is the only Slovak source
    found that describes spoken structural markers tested with blind
    students.
- Lambda 2015, the Slovak localization of the Lambda math editor as
  installed by the user (`C:\Lambda`, files dated 2015-12-07). The spoken
  strings are in `LambdaCode.xml`, block `<lang id="sk">`. Each element has a
  `name` and, for most structures, a verbose form (`open`, `sep`, `close`)
  and a brief form (`openb`, `sepb`, `closeb`). This is close to the
  ClearSpeak versus SimpleSpeak/Terse split. Elements without `open` are
  probably spoken by their `name`; that is inferred from the English block,
  not checked in the running program. `Lang/sk/LambdaMenu.xml` holds only the
  menu captions (names of the insert commands), and `Lang/sk/Messages.dat`
  holds user interface messages, not readings. The files come with the
  program and are not public; only single readings are quoted here.
  - What it covers: about 350 elements: letters, operators, relations, sets,
    logic, fractions, powers, roots, sums, products, limits, derivatives,
    integrals, matrices, arrows, functions, statistics.
  - Trust: medium. Slovak blind students used these strings with a real
    program, which counts for a lot. Who wrote this version and on what basis
    is unknown. It differs from the 2009 description in Horňanský 2009 (see
    Fractions below), so it is not certain that these are Horňanský's files.
    Several strings follow the English ones word for word ("je nerovnobežka
    so", "je dotyčnica"), and about eight are double-encoded ("zlomkovĂˇ
    čiara" for "zlomková čiara").
- vypocitat.sk, "Mocniny a odmocniny (kalkulačka)" (powers and roots
  calculator).
  URL: https://vypocitat.sk/mocniny-a-odmocniny-kalkulacka/
  - What it covers: the reading of a² and a³, and the phrase "druhá
    odmocnina z a".
  - Trust: medium-low. A popular educational site, but its readings match
    common school usage.
- Rebelova škola doma, "Mocniny a odmocniny" (powers and roots).
  URL: https://rebelovaskoladoma.estranky.sk/clanky/matika/mocniny-a-odmocniny.html
  - What it covers: the rule that the exponent is read as an ordinal in the
    accusative case.
  - Trust: medium-low (a home-schooling site). It is the only source found
    that states the grammatical rule explicitly.
- Pohodová matematika (Rudolf Zrebný), "Čo sú zlomky? Čítanie a zapisovanie
  zlomkov" (what fractions are; reading and writing them).
  URL: https://pohodovamatematika.sk/co-je-zlomok-citanie-a-zapisovanie-zlomkov.html
  - What it covers: reading fractions in words, and the "lomeno" form.
  - Trust: medium-low (personal educational site by a teacher). Useful
    because it discusses both reading styles.
- Slovak Wikipedia, articles "Mocnina", "Zlomok (matematika)" and "Limita".
  URLs: https://sk.wikipedia.org/wiki/Mocnina ,
  https://sk.wikipedia.org/wiki/Zlomok_(matematika) ,
  https://sk.wikipedia.org/wiki/Limita
  - What it covers: names of the parts (základ, exponent, mocniteľ, čitateľ,
    menovateľ, zlomková čiara), one reading of a^b, and the verbal form of a
    limit statement. The "Odmocnina" article has no reading.
  - Trust: medium for terminology. Little on spoken readings.
- encyklopedia.sk (Biblioteka.sk, a mirror of Slovak Wikipedia), "Derivácia
  (funkcia)" (derivative).
  URL: https://www.encyklopedia.sk/encyklopedia/?pojem=Deriv%C3%A1cia_%28funkcia%29
  - What it covers: the reading of the Leibniz notation.
  - Trust: medium.
- TUKE "Bach-Math", integral page by Miriama Drančáková, LF TUKE (Technical
  University of Košice), 2021.
  URL: https://peter.szabo.website.tuke.sk/bach-math/pages/integral.html
  - What it covers: the verbal description of a definite integral and of dx.
  - Trust: medium (university teaching material).
- Školská pomôcka (school worksheet), "Pojem druhá mocnina ..." (the notion
  of a square), ZŠ Skýcov, 2017.
  URL: https://skolaskycov.weblahko.sk/matematika/9_mocniny.pdf
  - What it covers: school terminology for powers and roots (druhá, tretia,
    n-tá mocnina and odmocnina; exponent = mocniteľ; základ odmocniny; znak
    odmocnenia).
  - Trust: medium for terms, at the level of grade 9 school use.
- mathematikos.sk, "Matematicko-slovenský slovník" (a math-to-Slovak
  dictionary).
  URL: https://mathematikos.sk/matematicko-slovensky_slovnik
  - What it covers: an informal explanation of the sum sign.
  - Trust: low. No reading of the full expression.
- Czech, secondary: Jedličková, Krupka, Nechvátalová, "Matematika – Výrazy a
  rovnice 1" (expressions and equations), Nová škola, 2016, an approved
  Czech textbook.
  URL: https://www.nns.cz/blog/wp-content/uploads/Matematika-Výrazy-a-rovnice-1.pdf
  - What it covers: explicit Czech readings of powers of fractions and of
    negative numbers, including the "to celé" (all of it) grouping phrase.
  - Trust: high for Czech. Only an analogy for Slovak.
- Czech, secondary, background only: Teiresiás, Masaryk University Brno.
  The poster "Technologie pro zpřístupnění výuky matematiky zrakově
  postiženým na MU" (technologies for making math teaching accessible to
  visually impaired students at MU).
  URL: https://www.teiresias.muni.cz/formin/documents/matematika%20final.pdf
  - What it covers: only the tools (BUF, Lambda, BlindMoose). No spoken
    readings.
- Sources named by the review group in the first feedback round (glossary
  package, 2026-11-16). Not yet researched in full; use them in the next
  rounds and check each before relying on it.
  - NVDA symbols.dic (above), confirmed in the local clone: `*` "hviezda",
    `(` "zátvorka", `)` "zatvoriť", arrows "šípka vľavo / hore / vpravo /
    dole". A reviewer says Slovak screen reader users are used to these
    short forms.
  - eSpeak NG Slovak dictionary, `dictsource/sk_list` (symbol names) in
    espeak-ng/espeak-ng.
    URL: https://github.com/espeak-ng/espeak-ng/blob/master/dictsource/sk_list
    - What it covers, per reviewers: "hviezda" for `*`. Also relevant as the
      synthesizer: eSpeak reads "ní" (Greek ν) as "ňí", so spelling choices
      must be tested with it (other synthesizers may do the same).
    - Trust: medium, same kind as NVDA (community translation, heard daily).
  - RHVoice, Slovak language data in RHVoice/RHVoice.
    URL: https://github.com/RHVoice/RHVoice
    - What it covers, per reviewers: "hviezda" for `*`. Exact file not yet
      located.
    - Trust: medium (community).
  - Unicode CLDR Slovak annotations, `common/annotations/sk.xml` in
    unicode-org/cldr (short names for symbols and emoji).
    URL: https://github.com/unicode-org/cldr/blob/main/common/annotations/sk.xml
    - What it covers: arrows "šípka doľava / doprava / nahor / nadol". Per a
      reviewer, U+2B05-U+2B07 (BLACK, filled arrows) and U+2190-U+2193 get
      the same Slovak names, so CLDR does not tell them apart.
    - Trust: medium-low for math. Who made the Slovak entries and on what
      basis is unknown, and the duplicate names lower confidence.
  - Lambda editor localization files: one or two XML files with the Slovak
    readings that Martin Horňanský made for his thesis (see Horňanský 2009
    above). A reviewer says they paid attention to the language side. The
    files are not public. Found on 2026-11-19 in the user's installed copy of
    Lambda; see "Lambda 2015" above.
    - Trust: medium; the most direct prior Slovak work on spoken math.
  - Slovak Wikipedia, "Hyperbolická funkcia", confirms the hyperbolic
    function names.
    URL: https://sk.wikipedia.org/wiki/Hyperbolick%C3%A1_funkcia
  - Not a source: one reviewer attached a ChatGPT answer on the arrows. It
    proposes "plná šípka doľava" for U+2B05 (filled) against "šípka doľava"
    for U+2190. Treat it as a reviewer's proposal, not as a reference.
- Checked with no useful result:
  - STN EN ISO 80000-2:2020. The normoff.gov.sk preview (record 130359)
    shows that the Slovak edition contains only the English text, so there
    are no official Slovak verbal equivalents from ISO 80000-2. The 2017
    edition and STN ISO 31-11 previews returned 404.
  - Access8Math (NVDA add-on) has no Slovak locale.
  - SKN Levoča web pages: no published rules for reading math in audio
    books.

## 2. Slovak readings found

Format: "English construct: Slovak reading (source)". "Unsourced suggestion"
marks an entry that no source states.

### Fractions

- Fraction, named parts: čitateľ, menovateľ, zlomková čiara (sk.wikipedia
  Zlomok)
- Compound fraction, spoken structure: "zložený zlomok" at the start
  (alternatively just "zlomok"), "zlomková čiara" in the middle, "koniec
  zlomku" at the end. Example: "Zložený zlomok a plus b Zlomková čiara a
  mínus b Koniec zlomku." (Horňanský 2009, table 4.2, Lambda Slovak voice
  output)
- Compound fraction in Lambda 2015: verbose "čitateľ ... menovateľ ...
  koniec zlomku", brief "čitateľ ... menovateľ ..." (no end word). This
  differs from the 2009 table: "menovateľ" instead of "zlomková čiara", and
  no "zložený zlomok" at the start. The English block has "the fraction
  numerator ... denominator ... end fraction", so the Slovak strings may be a
  translation of it. (Lambda 2015)
- Simple fraction a/b in Lambda 2015: the bar element is named "zlomková
  čiara" and has no other spoken string, so a/b is probably "a zlomková čiara
  b". (Lambda 2015)
- For comparison, the Czech block of the same file has "zlomek ... lomeno
  ... konec zlomku". The Slovak block does not use "lomeno". (Lambda 2015,
  Czech)
- Simple numeric fraction 1/2: jedna polovina; then dve, tri, štyri poloviny;
  päť, šesť ... polovín (Pohodová matematika)
- Other denominators: tretiny, štvrtiny, pätiny ... stotiny (Pohodová
  matematika)
- Fraction with a large denominator, for example 3/58: "tri lomeno
  päťdesiatosem", or "tri osem a päťdesiatiny" (Pohodová matematika)
- Vulgar fraction characters: ½ polovica, ⅓ tretina, ⅔ dve tretiny, ¼
  štvrtina, ¾ tri štvrtiny, ⅕ pätina, ⅚ päť šestín, ⅛ osmina, ⅜ tri osminy
  (NVDA sk symbols.dic). Note that NVDA says "polovica" while Pohodová
  matematika says "polovina".
- Fraction bar ∕: zlomková čiara; slash /: lomka (SABP 2021). NVDA reads ∕
  as "deleno" and / as "lomka".
- Division ÷: deleno (SABP 2021; NVDA)

### Powers

- a²: "a na druhú"; a³: "a na tretiu" (vypocitat.sk)
- a^b: "a na b-tu"; the result is called the "b-tá mocnina čísla a"
  (sk.wikipedia Mocnina)
- General rule: base, then "na", then the exponent as an ordinal number in
  the accusative case. "2⁶ čítame ako dva na šiestu" (Rebelova škola doma)
- In a formula: πr² is read "pí krát r na druhú" (SABP 2021, example
  "výpočet obsahu", area calculation)
- Parts: základ (mocniny) / mocnenec; exponent / mocniteľ (sk.wikipedia
  Mocnina; ZŠ Skýcov worksheet)
- Squared units: km² "kilometrov štvorcových" (SABP 2021)
- Czech analogue: (2/5)⁴ "dvě pětiny, to celé na čtvrtou"; 2⁴/5 "2 na
  čtvrtou lomeno pěti"; (−2)⁵ "minus dvě, to celé na pátou" (Czech, Nová
  škola 2016)
- Grouped power, Slovak form "to celé na ...": unsourced suggestion, by
  analogy with the Czech "to celé".
- Simple power: "umocnené na" (verbose and brief); power with a compound
  exponent: "umocnené na ... koniec mocniny"; superscript as such: "horný
  index" (Lambda 2015)

### Roots

- √a: "druhá odmocnina z a" (vypocitat.sk). √ alone: "druhá odmocnina"
  (NVDA)
- ∛ tretia odmocnina; ∜ štvrtá odmocnina (NVDA; it spells "štvrta" without
  the accent, which is a typo)
- n-th root: "n-tá odmocnina z čísla x" (sk.wikipedia Odmocnina, used in the
  definition); "n-tá odmocnina" (ZŠ Skýcov worksheet)
- Parts: odmocniteľ (the index); základ odmocniny (the radicand); znak
  odmocnenia (the root sign) (ZŠ Skýcov worksheet)
- End-of-root marker exists as a concept, "koniec odmocniny" (Horňanský
  2009). The spoken string itself is not given.
- Lambda 2015 has only names for roots: "jednoduchá odmocnina" (simple
  square root) and "zložená odmocnina" (compound root), with no start or end
  strings. (Lambda 2015)

### Indices (subscripts and superscripts)

- Braille markers: začiatok horného indexu (start of superscript), začiatok
  dolného indexu (start of subscript), koniec indexu (end of index) (SABP
  2021, section 13.1)
- Prime ′: indexová čiarka; ″ dvojitá indexová čiarka (NVDA). SABP 2021 has
  "indexová čiarka, uhlová ..." (the second word is truncated in the
  extracted text).
- Spoken subscript x₁ as "x jedna" or "x index jedna": unsourced
  suggestion.
- Index elements: "dolný index", "horný index", "ľavý dolný index", "ľavý
  horný index"; under and over scripts: "podtext", "nadtext"; primes:
  "apostrof", "dvojitý apostrof", "trojitý apostrof" (Lambda 2015)

### Absolute value

- Character |: zvislá čiara (vertical line) (NVDA)
- ‖: vektorová norma (vector norm) (NVDA)
- |x| as "absolútna hodnota x" or "absolútna hodnota z x": unsourced
  suggestion. The definition pages (vypocitat.sk, encyklopediapoznania.sk)
  name the concept "absolútna hodnota čísla" but give no reading of the
  notation.
- |x|: verbose "absolútna hodnota", brief "začiatok absolútnej hodnoty" (the
  brief form is the longer one, which looks swapped) (Lambda 2015)

### Functions

- Sinusoid ∿: sínusovka; composition ∘: operátor krúžok (NVDA)
- f(x) as "f od x" or "f z x": unsourced suggestion. No source found.
- Lambda uses "zo" before the argument of several operators: "determinant
  zo", "celá časť zo", "logaritmus so základom ... zo", and "zo" after sums,
  products and limits (below). It never uses "od" for function application.
  (Lambda 2015)
- Function names: sínus, kosínus, tangens, kotangens, sekans, kosekans,
  arkussínus ... arkuskosekans, "hyperbolický sínus" and so on; "logaritmus"
  (common log), "prirodzený logaritmus", "zložená funkcia" for ∘. (Lambda
  2015)

### Limits

- Verbal statement: "limita funkcie f v bode c ... je L" (sk.wikipedia
  Limita)
- Sequence language: the terms "sa blížia k" a value; "sa blíži do plus
  nekonečna" / "do mínus nekonečna" (galeje.sk worksheet "Limita
  postupnosti", Králiková)
- ∞: nekonečno (NVDA)
- ≐: "približuje sa k limitu" (NVDA, literal Unicode name, not a spoken lim
  form)
- lim over x→a as "limita pre x idúce k a" or "limita pre x blížiace sa k a":
  unsourced suggestion. No source states a reading of the lim notation.
- Limit in Lambda: "limita ... ide do ... zo" (verbose), "limita" (brief);
  also "dolná limita", "horná limita" (Lambda 2015)

### Derivatives

- dy/dx: "dy podľa dx" ("Tento zápis sa číta dy podľa dx") (encyklopedia.sk,
  Derivácia)
- d/dx f(x): "derivácia funkcie f(x) podľa premennej x" (from the search
  snippet of the TUKE fyzmattab page; not opened directly)
- Derivatives up to the third order are written with primes; from the
  fourth order an exponent in parentheses is used (STU lecture by
  Velichová, search snippet only)
- ∂: parciálna derivácia; ∇: nabla (NVDA)
- f′(x) as "f s čiarkou od x" or "f čiarka x": unsourced suggestion.
- Derivative with respect to x: verbose "derivácia ... podľa x", brief
  "derivácia ... d x"; partial: "parciálna derivácia ... podľa x"; "n-tá
  derivácia"; differential: verbose "podľa", brief "diferenciál" (Lambda 2015)

### Integrals

- Definite integral: "určitý integrál z funkcie f(x) na intervale 〈a,b〉,
  inak povedané aj od a po b"; dx is the "diferenciál premennej x"; "premenná
  integrácie je x" (TUKE Bach-Math)
- ∫ integrál; ∬ dvojitý integrál; ∭ trojitý integrál; ∮ krivkový integrál;
  ∯ plošný integrál; ∰ objemový integrál (NVDA)
- ∫ from a to b of f(x) dx, spoken as "integrál od a po b z f od x dx" or "...
  podľa x": unsourced suggestion. Only the "od a po b" part is sourced.
- Definite integral in Lambda: "integrál od ... po ... koniec integrálu"
  (verbose), "integrál od" (brief); "neurčitý integrál"; "krivkový integrál
  ... po krivke"; "integrál po uzavretej krivke okolo gama" (Lambda 2015)

### Sums and products

- ∑: súčet; ∏: súčin (SABP 2021). NVDA: ∑ "n-árny súčet", ∏ "n-árny súčin",
  ∐ "vedľajší súčin".
- ∑ sign: "znak ∑ hovorí, že ideme spočítavať"; i under the sign, n above it
  (mathematikos.sk; informal)
- Grouping in speech: "Tri krát súčet x plus štyri, koniec súčtu, sa rovná
  sedem" for 3(x+4)=7 (Horňanský 2009, Slovak rendering of the Aster
  example)
- ∑ for i from 1 to n, spoken as "suma pre i od 1 po n" or "súčet pre i od
  1 po n": unsourced suggestion.
- Sum in Lambda: "suma ... pre ... zo" (verbose; the parts are the lower
  limit, the upper limit and the summand), "suma" (brief); product "súčin ...
  pre ... zo" (Lambda 2015). Note: "pre" stands where English has "to",
  between the lower and upper limits.
- Matrix: "matica ... koniec matice"; separators "oddeľovač stĺpcov",
  "oddeľovač riadkov" (Lambda 2015)

### Operators

- +: plus; −: mínus; ±: plus mínus (SABP; NVDA). SABP section 6.2 writes
  "plus-mínus".
- ∓: mínus alebo plus (SABP); mínus plus (NVDA)
- ∙: krát a skalárny súčin (times, and scalar product); ×: krát, vektorový a
  karteziánsky súčin (times, vector and Cartesian product) (SABP 2021).
  NVDA reads both × and ⋅ as "krát".
- ∗: hviezdičkový operátor (asterisk operator) (SABP)
- ∣: delí; ∤: nedelí (SABP; NVDA)
- Decimal comma: desatinná čiarka. "3,14" is read "tri celých štrnásť
  stotín"; "10,50" is read "desať celých päťdesiat" (SABP 2021)
- Repeating decimal: "pätnásť celých dvestošestnásť tisícin s periódou
  dvestošestnásť" (SABP 2021)
- %: percento; ‰: promile; °: stupeň; ℃: stupeň Celzia (SABP 2021). NVDA
  reads ° as "stupňov".
- Lambda 2015: + plus, − mínus, times "krát", ÷ "deleno", ± and ∓
  "plus/mínus" and "mínus/plus" (spoken form of the slash unclear), %
  "percent", ‰ "promile", ! "faktoriál", degrees "stupne", thousands
  separator "tisíc", decimal comma "desatinná čiarka", repeating decimal
  "opakujúce sa nuly" (Lambda 2015)

### Relations

- =: rovná sa; ≠: nerovná sa (SABP; NVDA)
- <: menší ako; >: väčší ako (SABP). NVDA has just "menší" and "väčší".
- ≤: menší alebo rovný; ≥: väčší alebo rovný (SABP). NVDA: "menší alebo sa
  rovná", "väčší alebo sa rovná".
- ≶: menší ako alebo väčší ako; ≷: väčší ako alebo menší ako (SABP)
- ≈: je približne rovné (SABP). NVDA gives "takmer úplne sa rovná", which is
  less natural.
- ≅: približne sa rovná; ≡: totožné; ∼: podobný; ∝: úmerné; ≪: omnoho
  menší; ≫: omnoho väčší (NVDA)
- ∥: rovnobežka; ⊥: kolmica; ∠: uhol; ∟: pravý uhol (NVDA)
- ⇒: dvojitá šípka vpravo; → šípka vpravo (NVDA, character names, not
  "implikuje")
- Lambda 2015, verbose and brief forms: < "je menšie ako" / "menšie ako"; >
  "je väčšie ako" / "väčšie ako"; ≤ "je menšie ako alebo je rovné" / "menšie
  ako alebo rovné"; ≥ the same with "väčšie"; ≪ "je oveľa menšie ako" /
  "oveľa menšie ako"; ≫ likewise; = "rovná sa"; ≠ "nerovná sa"; ≈ "sa
  približne rovná"; ≅ "je kongruentné" / "kongruentné"; ≡ "je ekvivalentné" /
  "ekvivalentné"; ∝ "je úmerné" / "úmerné"; ≺ "predchádza", ≻ "nasleduje";
  ∣ "deliteľ" (Lambda 2015). The brief forms drop "je", as our SimpleSpeak
  does.
- Geometry in Lambda: ∥ "je rovnobežka so" / "rovnobežka"; ∦ "je
  nerovnobežka so" / "nerovnobežka"; ⊥ "je kolmica na" / "kolmica"; "je
  dotyčnica" / "dotyčnica"; ∠ "uhol"; "vektor"; "skalárny súčin"; "vektorový
  súčin" (Lambda 2015). These follow the English strings closely.
- Arrows in Lambda: "šípka doľava", "šípka doprava", "šípka hore", "šípka
  dolu" (brief: "doľava", "doprava", "hore", "dolu"); "obojstranná šípka";
  ⇒ "implikuje"; ⇔ "je logicky ekvivalentné" / "logicky ekvivalentné"
  (Lambda 2015)

### Sets and logic

- ∈: je prvkom; ∉: nie je prvkom; ∋: obsahuje ako člen (NVDA)
- ⊂: je podmnožina; ⊆: podmnožina alebo sa rovná; ⊃: je nadmnožina; ⊄: nie je
  podmnožina (NVDA)
- Lambda set operators: Zjednotenie, Množinový rozdiel, Vlastná podmnožina,
  Podmnožina (Horňanský 2009, table 4.3)
- ∪: zjednotenie; ∩: prienik; ∖: množinové mínus; ∁: doplnok (NVDA)
- ∅: prázdna množina (NVDA; SABP gives "priemer alebo prázdna množina",
  diameter or empty set)
- Number sets: ℕ prirodzené čísla, ℤ celé čísla, ℚ racionálne čísla, ℝ
  reálne čísla, ℂ komplexné čísla (NVDA)
- ∀: pre všetky; ∃: existuje; ∄: neexistuje (NVDA)
- ∧: logické a; ∨: logické alebo (NVDA)
- ∴: preto; ∵: pretože; ∎: koniec dôkazu (NVDA)
- Lambda 2015, verbose and brief forms: ∈ "je prvok množiny" / "je z
  množiny"; ∉ "nie je prvok množiny" / "nie je z množiny"; ⊆ "je obsiahnuté
  alebo sa rovná"; ⊂ "je obsiahnutá" (sic, feminine); ⊇ "obsahuje alebo sa
  rovná"; ⊃ "obsahuje"; ∪ "zjednotenie"; ∩ "prienik"; ∖ "mínus"; complement
  "doplnok"; "symetrická diferencia"; "Kartézsky súčin"; "potenčná množina";
  "kardinalita"; ∅ "prázdna množina". Indexed union and intersection:
  "zjednotenie ... do ... množiny", brief closing "zo". (Lambda 2015)
- Number sets in Lambda: ℕ "množina prirodzených čísel" / "prirodzené
  čísla", and the same pattern for ℤ, ℚ, ℝ, ℂ (Lambda 2015)
- Logic in Lambda: "logické a", "logické alebo", "negácia", "pre všetky",
  "existuje", "neexistuje", "existuje práve jedno", "vylučujúca disjunkcia",
  "pravda", "nepravda", "spor", "tautológia" (Lambda 2015)
- Brackets in Lambda: verbose "ľavá zátvorka ... pravá zátvorka", brief
  "zátvorka"; likewise "ľavá/pravá hranatá zátvorka" and "ľavá/pravá zložená
  zátvorka"; "uhlové zátvorky"; | "zvislá čiara"; ‖ "dvojitá čiara" (Lambda
  2015)

### Greek letters

From SABP 2021 (the lowercase table):

- α alfa, β beta, γ gama, δ delta, ε epsilon, ζ zéta, η éta, θ théta
- ι iota, κ kapa, λ lambda, μ mí, ν ní, ξ ksí, ο omikron, π pí
- ρ ró, σ sigma, ς "písmeno sigma na konci slova" (final sigma), τ tau
- υ upsilon, φ fí, χ chí, ψ psí, ω omega

Capital letters are introduced with the prefix for "veľké grécke písmeno"
(capital Greek letter). Speaking capitals as "veľké" plus the name (for
example "veľké sigma") is an unsourced suggestion.

Lambda 2015 writes several names without the long vowels: "zeta", "eta",
"theta" (with "th"), and "xí" for ξ; the others match SABP. Capitals agree
in gender with the name: "veľká gama", "veľká delta", "veľká sigma", "veľká
omega", but "veľké pí", "veľké fí", "veľké psí", "veľký omikron". Capital
Latin letters are "veľké b" and so on. (Lambda 2015)
