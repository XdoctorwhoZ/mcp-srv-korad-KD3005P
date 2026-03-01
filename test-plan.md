# Plan de Test Hardware - Alimentation Korad KD3005P

## Vue d'ensemble
Ce plan de test définit une suite complète de tests pour valider le bon fonctionnement de l'alimentation Korad KD3005P via le serveur MCP. Les tests couvrent les fonctionnalités de base, les cas limites, la sécurité et la robustesse.

## Spécifications de l'équipement
- **Modèle**: Korad KD3005P
- **Tension**: 0-30V
- **Courant**: 0-5A
- **Puissance**: 150W max
- **Interface**: USB/Série

## Prérequis
- Alimentation Korad KD3005P connectée via USB
- Serveur MCP fonctionnel
- Charge de test résistive (recommandé: résistances 10Ω/25W)  
- Multimètre pour validation externe (optionnel)

## Tests de Base

### TB001 - Découverte et Connexion des Périphériques
**Objectif**: Vérifier la détection automatique des alimentations connectées
**Procédure**:
1. Lister les périphériques disponibles
2. Vérifier que l'alimentation est détectée
3. Valider les informations du numéro de série

**Critères de succès**: 
- ✅ Au moins une alimentation détectée
- ✅ Numéro de série valide retourné

### TB002 - Création et Gestion d'Instance
**Objectif**: Tester la création et la gestion des instances nommées
**Procédure**:
1. Créer une instance avec un nom descriptif
2. Lister les instances actives
3. Vérifier le statut de l'instance
4. Tenter de créer une instance avec un mauvais numéro de série

**Critères de succès**:
- ✅ Instance créée avec succès
- ✅ Instance listée dans les instances actives
- ✅ Statut "running" confirmé
- ✅ Erreur appropriée pour mauvais numéro de série

### TB003 - Configuration des Paramètres de Base
**Objectif**: Tester les réglages de tension et courant
**Procédure**:
1. Régler tension à 5V, courant à 1A
2. Lire et vérifier les paramètres configurés
3. Modifier la tension à 12V
4. Modifier le courant à 2A
5. Vérifier chaque changement

**Critères de succès**:
- ✅ Paramètres configurés correctement
- ✅ Lecture conforme aux valeurs configurées
- ✅ Modifications appliquées avec succès

## Tests Fonctionnels

### TF001 - Activation et Désactivation de la Sortie
**Objectif**: Tester le contrôle de l'état de sortie
**Procédure**:
1. Configurer 3V, 0.5A
2. Activer la sortie
3. Vérifier l'état "on"
4. Désactiver la sortie
5. Vérifier l'état "off"

**Critères de succès**:
- ✅ Sortie s'active correctement
- ✅ État correctement rapporté
- ✅ Sortie se désactive correctement

### TF002 - Test de Rampe de Tension
**Objectif**: Valider le comportement lors de variations progressives
**Procédure**:
1. Configurer courant limite à 2A
2. Activer la sortie
3. Augmenter la tension par paliers: 1V → 5V → 10V → 15V
4. Vérifier chaque palier
5. Redescendre: 15V → 10V → 5V → 0V

**Critères de succès**:
- ✅ Chaque palier configuré correctement
- ✅ Valeurs lues conformes aux consignes
- ✅ Transitions stables

### TF003 - Test de Limite de Courant
**Objectif**: Vérifier le comportement en limitation de courant
**Procédure**:
1. Configurer 10V, limite courant 1A
2. Connecter charge résistive 5Ω (théoriquement 2A)
3. Activer la sortie
4. Vérifier que le courant est limité à 1A
5. Mesurer la tension de sortie effective

**Critères de succès**:
- ✅ Courant effectivement limité
- ✅ Tension ajustée automatiquement
- ✅ Mode limitation détecté

## Tests de Sécurité

### TS001 - Protection Contre Surtension
**Objectif**: Vérifier les mécanismes de protection
**Procédure**:
1. Tenter de configurer 35V (au-delà de la limite)
2. Tenter de configurer -5V (valeur négative)
3. Vérifier que les valeurs restent dans les limites

**Critères de succès**:
- ✅ Valeurs limitées aux spécifications
- ✅ Erreurs appropriées générées
- ✅ État de l'alimentation stable

### TS002 - Protection Contre Surcourant  
**Objectif**: Tester les limites de courant
**Procédure**:
1. Tenter de configurer 8A (au-delà de la limite)
2. Tenter de configurer -1A (valeur négative)
3. Vérifier le comportement

**Critères de succès**:
- ✅ Courant limité aux spécifications
- ✅ Erreurs appropriées générées

### TS003 - Séquence d'Arrêt d'Urgence
**Objectif**: Tester l'arrêt immédiat en cas de problème
**Procédure**:
1. Configurer 15V, 3A et activer
2. Désactiver immédiatement
3. Vérifier l'arrêt instantané
4. Relancer la séquence plusieurs fois

**Critères de succès**:
- ✅ Arrêt immédiat de la sortie
- ✅ État correctement mis à jour
- ✅ Reproductibilité validée

## Tests de Robustesse

### TR001 - Test de Stabilité Long Terme
**Objectif**: Valider la stabilité sur une période prolongée
**Procédure**:
1. Configurer 12V, 1A avec charge résistive
2. Activer pendant 30 minutes
3. Surveiller les variations de tension/courant
4. Enregistrer les données toutes les minutes

**Critères de succès**:
- ✅ Drift de tension < 1%
- ✅ Drift de courant < 2%
- ✅ Aucune déconnexion

### TR002 - Test de Cycles Marche/Arrêt
**Objectif**: Vérifier la robustesse aux cycles répétés
**Procédure**:
1. Configurer 5V, 1A
2. Effectuer 100 cycles on/off avec pause 1s
3. Vérifier l'intégrité après chaque cycle
4. Mesurer les temps de réponse

**Critères de succès**:
- ✅ Tous les cycles réussis
- ✅ Temps de réponse < 200ms
- ✅ Paramètres stables

### TR003 - Test de Gestion des Erreurs
**Objectif**: Valider la récupération après erreur
**Procédure**:
1. Simuler des déconnexions temporaires
2. Tenter des opérations sur instance inexistante  
3. Vérifier la récupération automatique
4. Tester la gestion d'erreurs multiples

**Critères de succès**:
- ✅ Erreurs détectées et rapportées
- ✅ Récupération automatique fonctionnelle
- ✅ État cohérent maintenu

## Tests de Performance

### TP001 - Temps de Réponse des Commandes
**Objectif**: Mesurer les performances système
**Procédure**:
1. Mesurer temps de réponse pour chaque commande MCP
2. Effectuer 50 mesures par commande
3. Calculer statistiques (min, max, moyenne, écart-type)

**Critères de succès**:
- ✅ Temps moyen < 100ms par commande
- ✅ 95% des commandes < 200ms
- ✅ Pas de timeout

### TP002 - Test de Charge Multiple
**Objectif**: Tester les performances avec plusieurs instances
**Procédure**:
1. Créer 3 instances simultanées (si plusieurs alimentations)
2. Effectuer des opérations parallèles
3. Mesurer l'impact sur les performances

**Critères de succès**:
- ✅ Toutes les instances opérationnelles
- ✅ Pas d'interférence entre instances
- ✅ Performance dégradée < 50%

## Format de Rapport de Test

### Structure du Rapport
```
# Rapport de Test - Korad KD3005P
Date: [DATE/HEURE]
Configuration: [DÉTAILS SYSTÈME]
Durée totale: [TEMPS]

## Résumé Exécutif
- Tests exécutés: X/Y
- Tests réussis: X
- Tests échoués: Y  
- Taux de réussite: Z%

## Détails des Tests
### [Catégorie - ID Test]
- Statut: ✅ PASS / ❌ FAIL
- Durée: Xs
- Détails: [description]
- Erreurs: [si applicable]

## Métriques de Performance
- Temps de réponse moyen: Xms
- Stabilité tension: ±X%
- Stabilité courant: ±X%

## Recommandations
[Recommandations d'amélioration]
```

### Seuils de Validation
- **Précision tension**: ±1% ou ±0.1V (le plus grand)
- **Précision courant**: ±2% ou ±0.01A (le plus grand)
- **Temps réponse**: < 200ms pour 95% des commandes
- **Stabilité long terme**: Drift < 1% sur 30 minutes

## Environnement de Test
- OS: Compatible Windows/Linux/macOS
- Driver USB: Installé et fonctionnel
- Serveur MCP: Version 0.1.0+
- Charges de test: Résistances de précision recommandées