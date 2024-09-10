# XML Schema definitions for DMN models

The following subdirectories contain XSDs for validating DMN models in versions:

- [**1.5**](./1.5) - version 1.5 of the DMN standard (newest)
- [**1.4**](./1.4) - version 1.4 of the DMN standard
- [**1.3**](./1.3) - version 1.3 of the DMN standard 

The DMN model, to be properly validated against specific standard version,
must have the `DMN_NAMESPACE`, `DMNDI_NAMESPACE`, `DC_NAMESPACE` and `DI_NAMESPACE`
values properly set, like shown below:  

```xml
<?xml version="1.0" encoding="UTF-8"?>
<definitions xmlns="DMN_NAMESPACE"
             xmlns:dmndi="DMNDI_NAMESPACE"
             xmlns:dc="DC_NAMESPACE"
             xmlns:di="DI_NAMESPACE"
             id="example"
             name="example">
</definitions>
```
