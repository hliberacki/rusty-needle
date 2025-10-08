# rusty-needle
**Find the needle in the haystack of requirements**

## Disclaimer

### Legal

This project is an independent, non-commercial experiment created purely for learning and exploration.  
It operates on JSON data **exported by** [Sphinx-Needs](https://useblocks.com/sphinx-needs/) and is intended solely to demonstrate how additional policy-based validation could be implemented on top of that data.  

**rusty-needle** does not claim ownership of, nor attempt to reimplement, any functionality, concepts, or intellectual property belonging to Sphinx-Needs or its authors.  
All rights to the original Sphinx-Needs project, its code, and its data structures remain entirely with the original authors and contributors.  

This project is developed independently, out of respect and appreciation for Sphinx-Needs, and should be viewed as a technical learning exercise, not as a competing or derivative product.

---

**rusty-needle** is a lightweight command-line tool that validates [Sphinx-Needs](https://useblocks.com/sphinx-needs/) JSON exports against custom policy rules.  
It extends the standard Sphinx-Needs validation with additional checks, allowing teams to enforce stricter or project-specific consistency in their requirements data.

---

## Motivation

Sphinx-Needs provides a robust framework for managing requirements within documentation.  
**rusty-needle** introduces an additional validation layer that allows users to define **policies** for enforcing project-specific rules.  

Typical use cases include:

- Ensuring each implementation (`impl`) is linked to at least one requirement.  
- Verifying that the author of a requirement does not approve or validate their own implementation.  
- Checking that every requirement is covered directly or indirectly by a test case.  

This approach helps maintain traceability, accountability, and compliance with internal standards or safety guidelines.

