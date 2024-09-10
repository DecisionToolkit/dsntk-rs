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

## Version 1.5

| Namespace        | Value                                        | Date       | Remarks   |
|------------------|----------------------------------------------|------------|-----------|
| DMN_NAMESPACE    | https://www.omg.org/spec/DMN/20230324/MODEL/ | 2023.03.24 | new       |
| DMNDI_NAMESPACE  | https://www.omg.org/spec/DMN/20230324/DMNDI/ | 2023.03.24 | new       |
| DC_NAMESPACE     | http://www.omg.org/spec/DMN/20180521/DC/     | 2018.05.21 | =1.4 =1.3 |
| DI_NAMESPACE     | http://www.omg.org/spec/DMN/20180521/DI/     | 2018.05.21 | =1.4 =1.3 |

## Version 1.4

| Namespace        | Value                                        | Date       | Remarks |
|------------------|----------------------------------------------|------------|---------|
| DMN_NAMESPACE    | https://www.omg.org/spec/DMN/20211108/MODEL/ | 2021.11.08 | new     |
| DMNDI_NAMESPACE  | https://www.omg.org/spec/DMN/20191111/DMNDI/ | 2019.11.11 | =1.3    |
| DC_NAMESPACE     | http://www.omg.org/spec/DMN/20180521/DC/     | 2018.05.21 | =1.3    |
| DI_NAMESPACE     | http://www.omg.org/spec/DMN/20180521/DI/     | 2018.05.21 | =1.3    |

## Version 1.3

| Namespace        | Value                                        | Date       | Remarks |
|------------------|----------------------------------------------|------------|---------|
| DMN_NAMESPACE    | https://www.omg.org/spec/DMN/20191111/MODEL/ | 2019.11.11 |         |
| DMNDI_NAMESPACE  | https://www.omg.org/spec/DMN/20191111/DMNDI/ | 2019.11.11 |         |
| DC_NAMESPACE     | http://www.omg.org/spec/DMN/20180521/DC/     | 2018.05.21 |         |
| DI_NAMESPACE     | http://www.omg.org/spec/DMN/20180521/DI/     | 2018.05.21 |         |
