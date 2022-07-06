<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>Get-List Users</name>
   <tag></tag>
   <elementGuidId>d8a67809-ec99-471a-b6fa-980faf2ec074</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <connectionTimeout>0</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent></httpBodyContent>
   <httpBodyType></httpBodyType>
   <katalonVersion>8.3.5</katalonVersion>
   <maxResponseSize>0</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>GET</restRequestMethod>
   <restUrl>https://reqres.in/api/users?page=2</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceEndpoint></soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>0</socketTimeout>
   <useServiceInfoFromWsdl>true</useServiceInfoFromWsdl>
   <verificationScript>import static org.assertj.core.api.Assertions.*

import com.kms.katalon.core.testobject.RequestObject
import com.kms.katalon.core.testobject.ResponseObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webservice.verification.WSResponseManager

import groovy.json.JsonSlurper
import internal.GlobalVariable as GlobalVariable

RequestObject request = WSResponseManager.getInstance().getCurrentRequest()

ResponseObject response = WSResponseManager.getInstance().getCurrentResponse()


WS.verifyResponseStatusCode(response, 200)

assertThat(response.getStatusCode()).isEqualTo(200)

WS.verifyElementPropertyValue(response, 'data[0].email', 'michael.lawson@reqres.in')

WS.verifyElementPropertyValue(response, 'data[0].email', &quot;michael.lawson@reqres.in&quot;)
WS.verifyElementPropertyValue(response, 'data[0].first_name', &quot;Michael&quot;)
WS.verifyElementPropertyValue(response, 'data[0].last_name', &quot;Lawson&quot;)
WS.verifyElementPropertyValue(response, 'data[1].id', 8)
WS.verifyElementPropertyValue(response, 'data[1].email', &quot;lindsay.ferguson@reqres.in&quot;)
WS.verifyElementPropertyValue(response, 'data[1].first_name', &quot;Lindsay&quot;)
WS.verifyElementPropertyValue(response, 'data[1].last_name', &quot;Ferguson&quot;)
WS.verifyElementPropertyValue(response, 'data[2].email', &quot;tobias.funke@reqres.in&quot;)
WS.verifyElementPropertyValue(response, 'data[2].first_name', &quot;Tobias&quot;)
WS.verifyElementPropertyValue(response, 'data[2].last_name', &quot;Funke&quot;)</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
