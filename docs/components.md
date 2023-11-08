# Components

## Overview

| Component     	 | Description                                               	 | Dependencies                     	 | Fallibilities                                                                                                 	 | Resolution                                                                                   	 |
|-----------------|-------------------------------------------------------------|------------------------------------|-----------------------------------------------------------------------------------------------------------------|------------------------------------------------------------------------------------------------|
| CtxManager    	 | Detects and manages context information                   	 | None                             	 | None. Does not fail when env. variables are missing                                                           	 | 	                                                                                              |
| DnsManager    	 | Resolves internal and external DNS requests               	 | CtxManager                       	 | Fails when cluster DNS_Server is unset.  Fails when internal dns resolver cannot be constructed               	 | Set the DNS_SERVER env variable in the cluster env                                           	 |
| CfgManager    	 | Manages configurations for service                        	 | CtxManager                       	 | None.  get_service_config returns a default svc config when service id is default                             	 | Set ServiceID to the correct value                                                           	 |
| SvcEnvManager 	 | Manages environment for services                          	 | CtxManager DnsManager            	 | None get_svc_host returns an error when 1) ServiceID is set to default 2) The service was not yet initialized 	 | 1) Set ServiceID to the correct value 2) Call the initializer before requesting the svc host 	 |
| SvcManager    	 | Manages all dependencies and initialization for a service 	 | CtxManager CfgManager DnsManager 	 | 	                                                                                                               | 	                                                                                              |

## CtxManager

Detects and manages context information

Dependencies: None

## CfgManager

Resolves internal and external DNS requests

Dependencies:

* CtxManager

## DnsManager

Resolves internal and external DNS requests

Dependencies:

* CtxManager

## SvcEnvManager

Manages environment for services

Dependencies:

* CtxManager
* DnsManager

## SvcManager

Manages all dependencies and initialization for a service

Dependencies:

* CtxManager
* CfgManager
* DnsManager
* SvcEnvManager