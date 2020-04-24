<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>update</name>
   <tag></tag>
   <elementGuidId>bde895a0-e1de-48f0-b5e6-223d3d76ed52</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n    \&quot;smart_connect\&quot;: {\n        \&quot;category\&quot;: {\n            \&quot;id\&quot;: 1,\n            \&quot;name\&quot;: \&quot;perjalanan dinas\&quot;\n        },\n        \&quot;amount\&quot;: 200000,\n        \&quot;products\&quot;: [\n            {\n                \&quot;type\&quot;: \&quot;reimbursement\&quot;,\n                \&quot;transaction_id\&quot;: 23,\n                \&quot;description\&quot;: \&quot;Makan siang di cimb\&quot;,\n                \&quot;ammount\&quot;: 20000,\n                \&quot;status\&quot;: \&quot;approved\&quot;,\n                \&quot;submitted_date\&quot;: \&quot;2019-08-02\&quot;\n            }\n        ]\n    },\n    \&quot;additional_data\&quot;: [\n        {\n            \&quot;id\&quot;: 56278,\n            \&quot;local_id\&quot;: 1570443960954,\n            \&quot;position\&quot;: 1,\n            \&quot;block_order\&quot;: 1,\n            \&quot;input_id\&quot;: 11817,\n            \&quot;value\&quot;: null,\n            \&quot;name\&quot;: \&quot;dddd\&quot;,\n            \&quot;type\&quot;: \&quot;text\&quot;,\n            \&quot;percentage\&quot;: 0,\n            \&quot;placeholder\&quot;: \&quot;\&quot;,\n            \&quot;is_mandatory\&quot;: false,\n            \&quot;is_delete\&quot;: false,\n            \&quot;value_id\&quot;: 0,\n            \&quot;case_id\&quot;: 0,\n            \&quot;child_name\&quot;: \&quot;\&quot;,\n            \&quot;child_min\&quot;: 0,\n            \&quot;child_max\&quot;: 0,\n            \&quot;is_can_similar\&quot;: false,\n            \&quot;is_child_mandatory\&quot;: false,\n            \&quot;is_allow_to_edit\&quot;: true,\n            \&quot;is_editable\&quot;: true,\n            \&quot;order_id\&quot;: 0,\n            \&quot;available_list\&quot;: [],\n            \&quot;map_type\&quot;: 1,\n            \&quot;longitude\&quot;: 0,\n            \&quot;latitude\&quot;: 0,\n            \&quot;multiple_value\&quot;: [],\n            \&quot;groups\&quot;: []\n        }\n    ],\n    \&quot;id\&quot; : 97,\n    \&quot;form_id\&quot;: 1886,\n    \&quot;local_id\&quot;: 1570443960955,\n    \&quot;name\&quot;: \&quot;123123123\&quot;,\n    \&quot;offline_created_date\&quot;: 1570443960955,\n    \&quot;timezone\&quot;: \&quot;Mon Oct 07 2019 17:26:00 GMT+0700 (Western Indonesia Time)\&quot;\n}&quot;,
  &quot;contentType&quot;: &quot;text/plain&quot;,
  &quot;charset&quot;: &quot;UTF-8&quot;
}</httpBodyContent>
   <httpBodyType>text</httpBodyType>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Content-Type</name>
      <type>Main</type>
      <value>application/json</value>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Authorization</name>
      <type>Main</type>
      <value>eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJpc3MiOiJqb2pvbm9taWMtand0LXNlcnZpY2UiLCJpYXQiOjE1NjY3OTIyNTYsImV4cCI6MTU5ODMyODI1Niwic3ViIjoxMzQ1OCwic2Vzc19pZCI6MTg0NDEwLCJ1c2VyIjp7ImlkIjoxMzQ1OCwiZW1haWwiOiJqb2pvcWFAbWFpbGluYXRvci5jb20iLCJjb21wYW55X2lkIjo0MTQsInVzZXJfY29tcGFueV9pZCI6MjM1M30sInNlc3Npb25fc2V0dGluZyI6MX0.IoYGjwN63VLYwEiBSbHcMqB_gRTrWCKM1nrCOZq548k</value>
   </httpHeaderProperties>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>${url}/update</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceFunction></soapServiceFunction>
   <variables>
      <defaultValue>GlobalVariable.urlAPI</defaultValue>
      <description></description>
      <id>71e818b8-664d-4c35-9433-015860f31e70</id>
      <masked>false</masked>
      <name>url</name>
   </variables>
   <verificationScript>import static org.assertj.core.api.Assertions.*

import com.kms.katalon.core.testobject.RequestObject
import com.kms.katalon.core.testobject.ResponseObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webservice.verification.WSResponseManager

import groovy.json.JsonSlurper
import internal.GlobalVariable as GlobalVariable

RequestObject request = WSResponseManager.getInstance().getCurrentRequest()

ResponseObject response = WSResponseManager.getInstance().getCurrentResponse()
</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
