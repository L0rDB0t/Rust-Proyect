# TOKEN TIMELOCK VAULT

Solana Anchor Smart Contract para bloquear tokens SPL a favor de un beneficiario hasta una fecha futura específica. Ideal para vesting, airdrops programados o seguros de custodia.

---

## 🚀 ¿QUÉ HACE ESTE CONTRATO?

- Permite al owner inicializar un "vault" con tokens SPL que solo podrán ser retirados por el beneficiario después de cierta fecha (`release_time`).
- El beneficiario puede liberar los tokens una sola vez, después de cumplido el tiempo.
- Proporciona seguridad en el manejo de cuentas y transferencias.

---

## 📦 INSTALACIÓN Y COMPILACIÓN

1. Instala Rust, Anchor y Solana CLI.
2. Clona este repo:
   ```bash
   git clone <URL_DE_TU_REPO>
   cd <NOMBRE_DEL_REPO>
   ```
3. Compila el contrato:
   ```bash
   anchor build
   ```

---

## ⚙️ USO

1. **Inicializar Vault:**  
   El owner transfiere tokens SPL al vault especificando beneficiario y fecha.
2. **Liberar tokens:**  
   El beneficiario llama a `release` después de la fecha límite.

---

## 🧑‍💻 ESTRUCTURA DE ARCHIVOS

- `programs/token_timelock_vault/src/lib.rs`: Contrato principal.
- `AUDITORIA.md`: Auditoría de seguridad y recomendaciones.
- `tests/`: Tests automáticos (opcional, recomendable).

---

## 🛡️ SEGURIDAD Y AUDITORÍA

Consulta `AUDITORIA.md` para ver el análisis de riesgos, recomendaciones y pruebas sugeridas.

---

## 📝 ID DEL CONTRATO

"AHuxcfexoU55eDhM8V5jk2U6ZJocjHVn8H5Y1RYm5qSC"

---

## 🤝 Autor

- L0RDB0T

---
