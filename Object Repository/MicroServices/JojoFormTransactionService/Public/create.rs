<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>create</name>
   <tag></tag>
   <elementGuidId>0de8fd1b-1673-4d07-a59f-6c5d8db1c4fe</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n  \&quot;additional_data\&quot;: [\n    {\n      \&quot;id\&quot;: 0,\n      \&quot;local_id\&quot;: 1587711422291,\n      \&quot;position\&quot;: 1,\n      \&quot;block_order\&quot;: 1,\n      \&quot;input_id\&quot;: 14820,\n      \&quot;source_id\&quot;: 0,\n      \&quot;value\&quot;: \&quot;sdfsf\&quot;,\n      \&quot;name\&quot;: \&quot;text\&quot;,\n      \&quot;type\&quot;: \&quot;text\&quot;,\n      \&quot;percentage\&quot;: 0,\n      \&quot;placeholder\&quot;: \&quot;input text\&quot;,\n      \&quot;is_mandatory\&quot;: true,\n      \&quot;is_delete\&quot;: false,\n      \&quot;is_back_date\&quot;: false,\n      \&quot;is_backdate\&quot;: false,\n      \&quot;is_can_access_gallery\&quot;: true,\n      \&quot;camera_use_type\&quot;: 0,\n      \&quot;value_id\&quot;: 0,\n      \&quot;case_id\&quot;: 0,\n      \&quot;child_name\&quot;: \&quot;\&quot;,\n      \&quot;child_min\&quot;: 0,\n      \&quot;child_max\&quot;: 0,\n      \&quot;is_can_similar\&quot;: false,\n      \&quot;is_child_mandatory\&quot;: false,\n      \&quot;is_allow_to_edit\&quot;: true,\n      \&quot;is_editable\&quot;: true,\n      \&quot;order_id\&quot;: 0,\n      \&quot;available_list\&quot;: [],\n      \&quot;map_type\&quot;: 1,\n      \&quot;longitude\&quot;: 0,\n      \&quot;latitude\&quot;: 0,\n      \&quot;multiple_value\&quot;: [],\n      \&quot;groups\&quot;: []\n    },\n    {\n      \&quot;id\&quot;: 0,\n      \&quot;local_id\&quot;: 1587711422291,\n      \&quot;position\&quot;: 1,\n      \&quot;block_order\&quot;: 1,\n      \&quot;input_id\&quot;: 14821,\n      \&quot;source_id\&quot;: 0,\n      \&quot;value\&quot;: null,\n      \&quot;name\&quot;: \&quot;long text\&quot;,\n      \&quot;type\&quot;: \&quot;descriptive\&quot;,\n      \&quot;percentage\&quot;: 0,\n      \&quot;placeholder\&quot;: \&quot;input long text\&quot;,\n      \&quot;is_mandatory\&quot;: false,\n      \&quot;is_delete\&quot;: false,\n      \&quot;is_back_date\&quot;: false,\n      \&quot;is_backdate\&quot;: false,\n      \&quot;is_can_access_gallery\&quot;: true,\n      \&quot;camera_use_type\&quot;: 0,\n      \&quot;value_id\&quot;: 0,\n      \&quot;case_id\&quot;: 0,\n      \&quot;child_name\&quot;: \&quot;\&quot;,\n      \&quot;child_min\&quot;: 0,\n      \&quot;child_max\&quot;: 0,\n      \&quot;is_can_similar\&quot;: false,\n      \&quot;is_child_mandatory\&quot;: false,\n      \&quot;is_allow_to_edit\&quot;: true,\n      \&quot;is_editable\&quot;: true,\n      \&quot;order_id\&quot;: 0,\n      \&quot;available_list\&quot;: [],\n      \&quot;map_type\&quot;: 1,\n      \&quot;longitude\&quot;: 0,\n      \&quot;latitude\&quot;: 0,\n      \&quot;multiple_value\&quot;: [],\n      \&quot;groups\&quot;: []\n    },\n    {\n      \&quot;id\&quot;: 0,\n      \&quot;local_id\&quot;: 1587711422291,\n      \&quot;position\&quot;: 1,\n      \&quot;block_order\&quot;: 1,\n      \&quot;input_id\&quot;: 14822,\n      \&quot;source_id\&quot;: 0,\n      \&quot;value\&quot;: null,\n      \&quot;name\&quot;: \&quot;number\&quot;,\n      \&quot;type\&quot;: \&quot;${type}\&quot;,\n      \&quot;percentage\&quot;: 0,\n      \&quot;placeholder\&quot;: \&quot;input number\&quot;,\n      \&quot;is_mandatory\&quot;: \&quot;${is_mandatory}\&quot;,\n      \&quot;is_delete\&quot;: false,\n      \&quot;is_back_date\&quot;: \&quot;${is_back_date}\&quot;,\n      \&quot;is_backdate\&quot;: false,\n      \&quot;is_can_access_gallery\&quot;: true,\n      \&quot;camera_use_type\&quot;: 0,\n      \&quot;value_id\&quot;: 0,\n      \&quot;case_id\&quot;: 0,\n      \&quot;child_name\&quot;: \&quot;\&quot;,\n      \&quot;child_min\&quot;: 0,\n      \&quot;child_max\&quot;: 0,\n      \&quot;is_can_similar\&quot;: \&quot;${is_can_similar}\&quot;,\n      \&quot;is_child_mandatory\&quot;: false,\n      \&quot;is_allow_to_edit\&quot;: true,\n      \&quot;is_editable\&quot;: \&quot;${is_editable}\&quot;,\n      \&quot;order_id\&quot;: 0,\n      \&quot;available_list\&quot;: [],\n      \&quot;map_type\&quot;: \&quot;${map_type}\&quot;,\n      \&quot;longitude\&quot;: 0,\n      \&quot;latitude\&quot;: 0,\n      \&quot;multiple_value\&quot;: [],\n      \&quot;groups\&quot;: []\n    },\n    {\n      \&quot;id\&quot;: 0,\n      \&quot;local_id\&quot;: 1587711422291,\n      \&quot;position\&quot;: 1,\n      \&quot;block_order\&quot;: 1,\n      \&quot;input_id\&quot;: 14823,\n      \&quot;source_id\&quot;: 0,\n      \&quot;value\&quot;: \&quot;\&quot;,\n      \&quot;name\&quot;: \&quot;allow bd\&quot;,\n      \&quot;type\&quot;: \&quot;date\&quot;,\n      \&quot;percentage\&quot;: 0,\n      \&quot;placeholder\&quot;: \&quot;select\&quot;,\n      \&quot;is_mandatory\&quot;: false,\n      \&quot;is_delete\&quot;: false,\n      \&quot;is_back_date\&quot;: true,\n      \&quot;is_backdate\&quot;: true,\n      \&quot;is_can_access_gallery\&quot;: true,\n      \&quot;camera_use_type\&quot;: 0,\n      \&quot;value_id\&quot;: 0,\n      \&quot;case_id\&quot;: 0,\n      \&quot;child_name\&quot;: \&quot;\&quot;,\n      \&quot;child_min\&quot;: 0,\n      \&quot;child_max\&quot;: 0,\n      \&quot;is_can_similar\&quot;: false,\n      \&quot;is_child_mandatory\&quot;: false,\n      \&quot;is_allow_to_edit\&quot;: true,\n      \&quot;is_editable\&quot;: true,\n      \&quot;order_id\&quot;: 0,\n      \&quot;available_list\&quot;: [],\n      \&quot;map_type\&quot;: 1,\n      \&quot;longitude\&quot;: 0,\n      \&quot;latitude\&quot;: 0,\n      \&quot;multiple_value\&quot;: [],\n      \&quot;groups\&quot;: []\n    },\n    {\n      \&quot;id\&quot;: 0,\n      \&quot;local_id\&quot;: 1587711422291,\n      \&quot;position\&quot;: 1,\n      \&quot;block_order\&quot;: 1,\n      \&quot;input_id\&quot;: 14824,\n      \&quot;source_id\&quot;: 0,\n      \&quot;value\&quot;: \&quot;\&quot;,\n      \&quot;name\&quot;: \&quot;disallow bd\&quot;,\n      \&quot;type\&quot;: \&quot;date\&quot;,\n      \&quot;percentage\&quot;: 0,\n      \&quot;placeholder\&quot;: \&quot;select\&quot;,\n      \&quot;is_mandatory\&quot;: false,\n      \&quot;is_delete\&quot;: false,\n      \&quot;is_back_date\&quot;: false,\n      \&quot;is_backdate\&quot;: false,\n      \&quot;is_can_access_gallery\&quot;: true,\n      \&quot;camera_use_type\&quot;: 0,\n      \&quot;value_id\&quot;: 0,\n      \&quot;case_id\&quot;: 0,\n      \&quot;child_name\&quot;: \&quot;\&quot;,\n      \&quot;child_min\&quot;: 0,\n      \&quot;child_max\&quot;: 0,\n      \&quot;is_can_similar\&quot;: false,\n      \&quot;is_child_mandatory\&quot;: false,\n      \&quot;is_allow_to_edit\&quot;: true,\n      \&quot;is_editable\&quot;: true,\n      \&quot;order_id\&quot;: 0,\n      \&quot;available_list\&quot;: [],\n      \&quot;map_type\&quot;: 1,\n      \&quot;longitude\&quot;: 0,\n      \&quot;latitude\&quot;: 0,\n      \&quot;multiple_value\&quot;: [],\n      \&quot;groups\&quot;: []\n    },\n    {\n      \&quot;id\&quot;: 0,\n      \&quot;local_id\&quot;: 1587711422292,\n      \&quot;position\&quot;: 1,\n      \&quot;block_order\&quot;: 1,\n      \&quot;input_id\&quot;: 14825,\n      \&quot;source_id\&quot;: 0,\n      \&quot;value\&quot;: null,\n      \&quot;name\&quot;: \&quot;time\&quot;,\n      \&quot;type\&quot;: \&quot;time\&quot;,\n      \&quot;percentage\&quot;: 0,\n      \&quot;placeholder\&quot;: \&quot;select\&quot;,\n      \&quot;is_mandatory\&quot;: false,\n      \&quot;is_delete\&quot;: false,\n      \&quot;is_back_date\&quot;: false,\n      \&quot;is_backdate\&quot;: false,\n      \&quot;is_can_access_gallery\&quot;: true,\n      \&quot;camera_use_type\&quot;: 0,\n      \&quot;value_id\&quot;: 0,\n      \&quot;case_id\&quot;: 0,\n      \&quot;child_name\&quot;: \&quot;\&quot;,\n      \&quot;child_min\&quot;: 0,\n      \&quot;child_max\&quot;: 0,\n      \&quot;is_can_similar\&quot;: false,\n      \&quot;is_child_mandatory\&quot;: false,\n      \&quot;is_allow_to_edit\&quot;: true,\n      \&quot;is_editable\&quot;: true,\n      \&quot;order_id\&quot;: 0,\n      \&quot;available_list\&quot;: [],\n      \&quot;map_type\&quot;: 1,\n      \&quot;longitude\&quot;: 0,\n      \&quot;latitude\&quot;: 0,\n      \&quot;multiple_value\&quot;: [],\n      \&quot;groups\&quot;: []\n    },\n    {\n      \&quot;id\&quot;: 0,\n      \&quot;local_id\&quot;: 1587711422292,\n      \&quot;position\&quot;: 1,\n      \&quot;block_order\&quot;: 1,\n      \&quot;input_id\&quot;: 14826,\n      \&quot;source_id\&quot;: 0,\n      \&quot;value\&quot;: \&quot;\&quot;,\n      \&quot;name\&quot;: \&quot;current loc\&quot;,\n      \&quot;type\&quot;: \&quot;location\&quot;,\n      \&quot;percentage\&quot;: 0,\n      \&quot;placeholder\&quot;: \&quot;select\&quot;,\n      \&quot;is_mandatory\&quot;: false,\n      \&quot;is_delete\&quot;: false,\n      \&quot;is_back_date\&quot;: false,\n      \&quot;is_backdate\&quot;: false,\n      \&quot;is_can_access_gallery\&quot;: true,\n      \&quot;camera_use_type\&quot;: 0,\n      \&quot;value_id\&quot;: 0,\n      \&quot;case_id\&quot;: 0,\n      \&quot;child_name\&quot;: \&quot;\&quot;,\n      \&quot;child_min\&quot;: 0,\n      \&quot;child_max\&quot;: 0,\n      \&quot;is_can_similar\&quot;: false,\n      \&quot;is_child_mandatory\&quot;: false,\n      \&quot;is_allow_to_edit\&quot;: true,\n      \&quot;is_editable\&quot;: true,\n      \&quot;order_id\&quot;: 0,\n      \&quot;available_list\&quot;: [],\n      \&quot;map_type\&quot;: 1,\n      \&quot;longitude\&quot;: 0,\n      \&quot;latitude\&quot;: 0,\n      \&quot;multiple_value\&quot;: [],\n      \&quot;groups\&quot;: []\n    },\n    {\n      \&quot;id\&quot;: 0,\n      \&quot;local_id\&quot;: 1587711422292,\n      \&quot;position\&quot;: 1,\n      \&quot;block_order\&quot;: 1,\n      \&quot;input_id\&quot;: 14827,\n      \&quot;source_id\&quot;: 0,\n      \&quot;value\&quot;: \&quot;\&quot;,\n      \&quot;name\&quot;: \&quot;manual loc\&quot;,\n      \&quot;type\&quot;: \&quot;location\&quot;,\n      \&quot;percentage\&quot;: 0,\n      \&quot;placeholder\&quot;: \&quot;select\&quot;,\n      \&quot;is_mandatory\&quot;: false,\n      \&quot;is_delete\&quot;: false,\n      \&quot;is_back_date\&quot;: false,\n      \&quot;is_backdate\&quot;: false,\n      \&quot;is_can_access_gallery\&quot;: true,\n      \&quot;camera_use_type\&quot;: 0,\n      \&quot;value_id\&quot;: 0,\n      \&quot;case_id\&quot;: 0,\n      \&quot;child_name\&quot;: \&quot;\&quot;,\n      \&quot;child_min\&quot;: 0,\n      \&quot;child_max\&quot;: 0,\n      \&quot;is_can_similar\&quot;: false,\n      \&quot;is_child_mandatory\&quot;: false,\n      \&quot;is_allow_to_edit\&quot;: true,\n      \&quot;is_editable\&quot;: true,\n      \&quot;order_id\&quot;: 0,\n      \&quot;available_list\&quot;: [],\n      \&quot;map_type\&quot;: 2,\n      \&quot;longitude\&quot;: 0,\n      \&quot;latitude\&quot;: 0,\n      \&quot;multiple_value\&quot;: [],\n      \&quot;groups\&quot;: []\n    },\n    {\n      \&quot;id\&quot;: 0,\n      \&quot;local_id\&quot;: 1587711422292,\n      \&quot;position\&quot;: 1,\n      \&quot;block_order\&quot;: 1,\n      \&quot;input_id\&quot;: 14828,\n      \&quot;source_id\&quot;: 0,\n      \&quot;value\&quot;: \&quot;\&quot;,\n      \&quot;name\&quot;: \&quot;all loc\&quot;,\n      \&quot;type\&quot;: \&quot;location\&quot;,\n      \&quot;percentage\&quot;: 0,\n      \&quot;placeholder\&quot;: \&quot;select\&quot;,\n      \&quot;is_mandatory\&quot;: false,\n      \&quot;is_delete\&quot;: false,\n      \&quot;is_back_date\&quot;: false,\n      \&quot;is_backdate\&quot;: false,\n      \&quot;is_can_access_gallery\&quot;: true,\n      \&quot;camera_use_type\&quot;: 0,\n      \&quot;value_id\&quot;: 0,\n      \&quot;case_id\&quot;: 0,\n      \&quot;child_name\&quot;: \&quot;\&quot;,\n      \&quot;child_min\&quot;: 0,\n      \&quot;child_max\&quot;: 0,\n      \&quot;is_can_similar\&quot;: false,\n      \&quot;is_child_mandatory\&quot;: false,\n      \&quot;is_allow_to_edit\&quot;: true,\n      \&quot;is_editable\&quot;: true,\n      \&quot;order_id\&quot;: 0,\n      \&quot;available_list\&quot;: [],\n      \&quot;map_type\&quot;: 3,\n      \&quot;longitude\&quot;: 0,\n      \&quot;latitude\&quot;: 0,\n      \&quot;multiple_value\&quot;: [],\n      \&quot;groups\&quot;: []\n    },\n    {\n      \&quot;id\&quot;: 0,\n      \&quot;local_id\&quot;: 1587711422292,\n      \&quot;position\&quot;: 1,\n      \&quot;block_order\&quot;: 1,\n      \&quot;input_id\&quot;: 14829,\n      \&quot;source_id\&quot;: 1415,\n      \&quot;value\&quot;: \&quot;\&quot;,\n      \&quot;name\&quot;: \&quot;both back\&quot;,\n      \&quot;type\&quot;: \&quot;photo\&quot;,\n      \&quot;percentage\&quot;: 0,\n      \&quot;placeholder\&quot;: \&quot;select\&quot;,\n      \&quot;is_mandatory\&quot;: false,\n      \&quot;is_delete\&quot;: false,\n      \&quot;is_back_date\&quot;: false,\n      \&quot;is_backdate\&quot;: false,\n      \&quot;is_can_access_gallery\&quot;: true,\n      \&quot;camera_use_type\&quot;: 1,\n      \&quot;value_id\&quot;: 0,\n      \&quot;case_id\&quot;: 0,\n      \&quot;child_name\&quot;: \&quot;\&quot;,\n      \&quot;child_min\&quot;: 0,\n      \&quot;child_max\&quot;: 0,\n      \&quot;is_can_similar\&quot;: false,\n      \&quot;is_child_mandatory\&quot;: false,\n      \&quot;is_allow_to_edit\&quot;: true,\n      \&quot;is_editable\&quot;: true,\n      \&quot;order_id\&quot;: 0,\n      \&quot;available_list\&quot;: [],\n      \&quot;map_type\&quot;: 1,\n      \&quot;longitude\&quot;: 0,\n      \&quot;latitude\&quot;: 0,\n      \&quot;multiple_value\&quot;: [\n        {\n          \&quot;amount\&quot;: 0,\n          \&quot;available_childs\&quot;: [],\n          \&quot;description\&quot;: \&quot;\&quot;,\n          \&quot;id\&quot;: 9981,\n          \&quot;name\&quot;: \&quot;Times\&quot;,\n          \&quot;url\&quot;: \&quot;\&quot;\n        },\n        {\n          \&quot;amount\&quot;: 0,\n          \&quot;available_childs\&quot;: [],\n          \&quot;description\&quot;: \&quot;\&quot;,\n          \&quot;id\&quot;: 9982,\n          \&quot;name\&quot;: \&quot;Expense\&quot;,\n          \&quot;url\&quot;: \&quot;\&quot;\n        },\n        {\n          \&quot;amount\&quot;: 0,\n          \&quot;available_childs\&quot;: [],\n          \&quot;description\&quot;: \&quot;\&quot;,\n          \&quot;id\&quot;: 9983,\n          \&quot;name\&quot;: \&quot;Procure\&quot;,\n          \&quot;url\&quot;: \&quot;\&quot;\n        }\n      ],\n      \&quot;groups\&quot;: []\n    },\n    {\n      \&quot;id\&quot;: 0,\n      \&quot;local_id\&quot;: 1587711422292,\n      \&quot;position\&quot;: 1,\n      \&quot;block_order\&quot;: 1,\n      \&quot;input_id\&quot;: 14830,\n      \&quot;source_id\&quot;: 1415,\n      \&quot;value\&quot;: \&quot;\&quot;,\n      \&quot;name\&quot;: \&quot;gallery only\&quot;,\n      \&quot;type\&quot;: \&quot;photo\&quot;,\n      \&quot;percentage\&quot;: 0,\n      \&quot;placeholder\&quot;: \&quot;select\&quot;,\n      \&quot;is_mandatory\&quot;: false,\n      \&quot;is_delete\&quot;: false,\n      \&quot;is_back_date\&quot;: false,\n      \&quot;is_backdate\&quot;: false,\n      \&quot;is_can_access_gallery\&quot;: true,\n      \&quot;camera_use_type\&quot;: 0,\n      \&quot;value_id\&quot;: 0,\n      \&quot;case_id\&quot;: 0,\n      \&quot;child_name\&quot;: \&quot;\&quot;,\n      \&quot;child_min\&quot;: 0,\n      \&quot;child_max\&quot;: 0,\n      \&quot;is_can_similar\&quot;: false,\n      \&quot;is_child_mandatory\&quot;: false,\n      \&quot;is_allow_to_edit\&quot;: true,\n      \&quot;is_editable\&quot;: true,\n      \&quot;order_id\&quot;: 0,\n      \&quot;available_list\&quot;: [],\n      \&quot;map_type\&quot;: 1,\n      \&quot;longitude\&quot;: 0,\n      \&quot;latitude\&quot;: 0,\n      \&quot;multiple_value\&quot;: [\n        {\n          \&quot;amount\&quot;: 0,\n          \&quot;available_childs\&quot;: [],\n          \&quot;description\&quot;: \&quot;\&quot;,\n          \&quot;id\&quot;: 9981,\n          \&quot;name\&quot;: \&quot;Times\&quot;,\n          \&quot;url\&quot;: \&quot;\&quot;\n        },\n        {\n          \&quot;amount\&quot;: 0,\n          \&quot;available_childs\&quot;: [],\n          \&quot;description\&quot;: \&quot;\&quot;,\n          \&quot;id\&quot;: 9982,\n          \&quot;name\&quot;: \&quot;Expense\&quot;,\n          \&quot;url\&quot;: \&quot;\&quot;\n        },\n        {\n          \&quot;amount\&quot;: 0,\n          \&quot;available_childs\&quot;: [],\n          \&quot;description\&quot;: \&quot;\&quot;,\n          \&quot;id\&quot;: 9983,\n          \&quot;name\&quot;: \&quot;Procure\&quot;,\n          \&quot;url\&quot;: \&quot;\&quot;\n        }\n      ],\n      \&quot;groups\&quot;: []\n    },\n    {\n      \&quot;id\&quot;: 0,\n      \&quot;local_id\&quot;: 1587711422293,\n      \&quot;position\&quot;: 1,\n      \&quot;block_order\&quot;: 1,\n      \&quot;input_id\&quot;: 14831,\n      \&quot;source_id\&quot;: 1415,\n      \&quot;value\&quot;: \&quot;\&quot;,\n      \&quot;name\&quot;: \&quot;front cam only\&quot;,\n      \&quot;type\&quot;: \&quot;photo\&quot;,\n      \&quot;percentage\&quot;: 0,\n      \&quot;placeholder\&quot;: \&quot;select\&quot;,\n      \&quot;is_mandatory\&quot;: false,\n      \&quot;is_delete\&quot;: false,\n      \&quot;is_back_date\&quot;: false,\n      \&quot;is_backdate\&quot;: false,\n      \&quot;is_can_access_gallery\&quot;: false,\n      \&quot;camera_use_type\&quot;: 2,\n      \&quot;value_id\&quot;: 0,\n      \&quot;case_id\&quot;: 0,\n      \&quot;child_name\&quot;: \&quot;\&quot;,\n      \&quot;child_min\&quot;: 0,\n      \&quot;child_max\&quot;: 0,\n      \&quot;is_can_similar\&quot;: false,\n      \&quot;is_child_mandatory\&quot;: false,\n      \&quot;is_allow_to_edit\&quot;: true,\n      \&quot;is_editable\&quot;: true,\n      \&quot;order_id\&quot;: 0,\n      \&quot;available_list\&quot;: [],\n      \&quot;map_type\&quot;: 1,\n      \&quot;longitude\&quot;: 0,\n      \&quot;latitude\&quot;: 0,\n      \&quot;multiple_value\&quot;: [\n        {\n          \&quot;amount\&quot;: 0,\n          \&quot;available_childs\&quot;: [],\n          \&quot;description\&quot;: \&quot;\&quot;,\n          \&quot;id\&quot;: 9981,\n          \&quot;name\&quot;: \&quot;Times\&quot;,\n          \&quot;url\&quot;: \&quot;\&quot;\n        },\n        {\n          \&quot;amount\&quot;: 0,\n          \&quot;available_childs\&quot;: [],\n          \&quot;description\&quot;: \&quot;\&quot;,\n          \&quot;id\&quot;: 9982,\n          \&quot;name\&quot;: \&quot;Expense\&quot;,\n          \&quot;url\&quot;: \&quot;\&quot;\n        },\n        {\n          \&quot;amount\&quot;: 0,\n          \&quot;available_childs\&quot;: [],\n          \&quot;description\&quot;: \&quot;\&quot;,\n          \&quot;id\&quot;: 9983,\n          \&quot;name\&quot;: \&quot;Procure\&quot;,\n          \&quot;url\&quot;: \&quot;\&quot;\n        }\n      ],\n      \&quot;groups\&quot;: []\n    },\n    {\n      \&quot;id\&quot;: 0,\n      \&quot;local_id\&quot;: 1587711422293,\n      \&quot;position\&quot;: 1,\n      \&quot;block_order\&quot;: 1,\n      \&quot;input_id\&quot;: 14832,\n      \&quot;source_id\&quot;: 1415,\n      \&quot;value\&quot;: \&quot;\&quot;,\n      \&quot;name\&quot;: \&quot;all cam only\&quot;,\n      \&quot;type\&quot;: \&quot;photo\&quot;,\n      \&quot;percentage\&quot;: 0,\n      \&quot;placeholder\&quot;: \&quot;select\&quot;,\n      \&quot;is_mandatory\&quot;: false,\n      \&quot;is_delete\&quot;: false,\n      \&quot;is_back_date\&quot;: false,\n      \&quot;is_backdate\&quot;: false,\n      \&quot;is_can_access_gallery\&quot;: false,\n      \&quot;camera_use_type\&quot;: 3,\n      \&quot;value_id\&quot;: 0,\n      \&quot;case_id\&quot;: 0,\n      \&quot;child_name\&quot;: \&quot;\&quot;,\n      \&quot;child_min\&quot;: 0,\n      \&quot;child_max\&quot;: 0,\n      \&quot;is_can_similar\&quot;: false,\n      \&quot;is_child_mandatory\&quot;: false,\n      \&quot;is_allow_to_edit\&quot;: true,\n      \&quot;is_editable\&quot;: true,\n      \&quot;order_id\&quot;: 0,\n      \&quot;available_list\&quot;: [],\n      \&quot;map_type\&quot;: 1,\n      \&quot;longitude\&quot;: 0,\n      \&quot;latitude\&quot;: 0,\n      \&quot;multiple_value\&quot;: [\n        {\n          \&quot;amount\&quot;: 0,\n          \&quot;available_childs\&quot;: [],\n          \&quot;description\&quot;: \&quot;\&quot;,\n          \&quot;id\&quot;: 9981,\n          \&quot;name\&quot;: \&quot;Times\&quot;,\n          \&quot;url\&quot;: \&quot;\&quot;\n        },\n        {\n          \&quot;amount\&quot;: 0,\n          \&quot;available_childs\&quot;: [],\n          \&quot;description\&quot;: \&quot;\&quot;,\n          \&quot;id\&quot;: 9982,\n          \&quot;name\&quot;: \&quot;Expense\&quot;,\n          \&quot;url\&quot;: \&quot;\&quot;\n        },\n        {\n          \&quot;amount\&quot;: 0,\n          \&quot;available_childs\&quot;: [],\n          \&quot;description\&quot;: \&quot;\&quot;,\n          \&quot;id\&quot;: 9983,\n          \&quot;name\&quot;: \&quot;Procure\&quot;,\n          \&quot;url\&quot;: \&quot;\&quot;\n        }\n      ],\n      \&quot;groups\&quot;: []\n    },\n    {\n      \&quot;id\&quot;: 0,\n      \&quot;local_id\&quot;: 1587711422293,\n      \&quot;position\&quot;: 1,\n      \&quot;block_order\&quot;: 1,\n      \&quot;input_id\&quot;: 14833,\n      \&quot;source_id\&quot;: 1415,\n      \&quot;value\&quot;: \&quot;\&quot;,\n      \&quot;name\&quot;: \&quot;single select\&quot;,\n      \&quot;type\&quot;: \&quot;single_selection\&quot;,\n      \&quot;percentage\&quot;: 0,\n      \&quot;placeholder\&quot;: \&quot;select\&quot;,\n      \&quot;is_mandatory\&quot;: false,\n      \&quot;is_delete\&quot;: false,\n      \&quot;is_back_date\&quot;: false,\n      \&quot;is_backdate\&quot;: false,\n      \&quot;is_can_access_gallery\&quot;: true,\n      \&quot;camera_use_type\&quot;: 0,\n      \&quot;value_id\&quot;: 0,\n      \&quot;case_id\&quot;: 0,\n      \&quot;child_name\&quot;: \&quot;\&quot;,\n      \&quot;child_min\&quot;: 0,\n      \&quot;child_max\&quot;: 0,\n      \&quot;is_can_similar\&quot;: false,\n      \&quot;is_child_mandatory\&quot;: false,\n      \&quot;is_allow_to_edit\&quot;: true,\n      \&quot;is_editable\&quot;: true,\n      \&quot;order_id\&quot;: 0,\n      \&quot;available_list\&quot;: [\n        {\n          \&quot;_id\&quot;: \&quot;5d1da49dbb7bb06ae064b398\&quot;,\n          \&quot;id\&quot;: 9981,\n          \&quot;name\&quot;: \&quot;Times\&quot;,\n          \&quot;url\&quot;: \&quot;\&quot;,\n          \&quot;amount\&quot;: 0,\n          \&quot;description\&quot;: \&quot;\&quot;,\n          \&quot;related_source_id\&quot;: 0,\n          \&quot;available_childs\&quot;: [],\n          \&quot;label\&quot;: \&quot;Times\&quot;\n        },\n        {\n          \&quot;_id\&quot;: \&quot;5d1da49dbb7bb06ae064b399\&quot;,\n          \&quot;id\&quot;: 9982,\n          \&quot;name\&quot;: \&quot;Expense\&quot;,\n          \&quot;url\&quot;: \&quot;\&quot;,\n          \&quot;amount\&quot;: 0,\n          \&quot;description\&quot;: \&quot;\&quot;,\n          \&quot;related_source_id\&quot;: 0,\n          \&quot;available_childs\&quot;: [],\n          \&quot;label\&quot;: \&quot;Expense\&quot;\n        },\n        {\n          \&quot;_id\&quot;: \&quot;5d1da49dbb7bb06ae064b39a\&quot;,\n          \&quot;id\&quot;: 9983,\n          \&quot;name\&quot;: \&quot;Procure\&quot;,\n          \&quot;url\&quot;: \&quot;\&quot;,\n          \&quot;amount\&quot;: 0,\n          \&quot;description\&quot;: \&quot;\&quot;,\n          \&quot;related_source_id\&quot;: 0,\n          \&quot;available_childs\&quot;: [],\n          \&quot;label\&quot;: \&quot;Procure\&quot;\n        }\n      ],\n      \&quot;map_type\&quot;: 1,\n      \&quot;longitude\&quot;: 0,\n      \&quot;latitude\&quot;: 0,\n      \&quot;multiple_value\&quot;: [],\n      \&quot;groups\&quot;: []\n    },\n    {\n      \&quot;id\&quot;: 0,\n      \&quot;local_id\&quot;: 1587711422293,\n      \&quot;position\&quot;: 1,\n      \&quot;block_order\&quot;: 1,\n      \&quot;input_id\&quot;: 14834,\n      \&quot;source_id\&quot;: 1415,\n      \&quot;value\&quot;: \&quot;\&quot;,\n      \&quot;name\&quot;: \&quot;multi duplicate\&quot;,\n      \&quot;type\&quot;: \&quot;multiple_selection\&quot;,\n      \&quot;percentage\&quot;: 0,\n      \&quot;placeholder\&quot;: \&quot;select\&quot;,\n      \&quot;is_mandatory\&quot;: false,\n      \&quot;is_delete\&quot;: false,\n      \&quot;is_back_date\&quot;: false,\n      \&quot;is_backdate\&quot;: false,\n      \&quot;is_can_access_gallery\&quot;: true,\n      \&quot;camera_use_type\&quot;: 0,\n      \&quot;value_id\&quot;: 0,\n      \&quot;case_id\&quot;: 0,\n      \&quot;child_name\&quot;: \&quot;\&quot;,\n      \&quot;child_min\&quot;: 0,\n      \&quot;child_max\&quot;: 0,\n      \&quot;is_can_similar\&quot;: true,\n      \&quot;is_child_mandatory\&quot;: false,\n      \&quot;is_allow_to_edit\&quot;: true,\n      \&quot;is_editable\&quot;: true,\n      \&quot;order_id\&quot;: 0,\n      \&quot;available_list\&quot;: [\n        {\n          \&quot;_id\&quot;: \&quot;5d1da49dbb7bb06ae064b398\&quot;,\n          \&quot;id\&quot;: 9981,\n          \&quot;name\&quot;: \&quot;Times\&quot;,\n          \&quot;url\&quot;: \&quot;\&quot;,\n          \&quot;amount\&quot;: 0,\n          \&quot;description\&quot;: \&quot;\&quot;,\n          \&quot;related_source_id\&quot;: 0,\n          \&quot;available_childs\&quot;: [],\n          \&quot;label\&quot;: \&quot;Times\&quot;\n        },\n        {\n          \&quot;_id\&quot;: \&quot;5d1da49dbb7bb06ae064b399\&quot;,\n          \&quot;id\&quot;: 9982,\n          \&quot;name\&quot;: \&quot;Expense\&quot;,\n          \&quot;url\&quot;: \&quot;\&quot;,\n          \&quot;amount\&quot;: 0,\n          \&quot;description\&quot;: \&quot;\&quot;,\n          \&quot;related_source_id\&quot;: 0,\n          \&quot;available_childs\&quot;: [],\n          \&quot;label\&quot;: \&quot;Expense\&quot;\n        },\n        {\n          \&quot;_id\&quot;: \&quot;5d1da49dbb7bb06ae064b39a\&quot;,\n          \&quot;id\&quot;: 9983,\n          \&quot;name\&quot;: \&quot;Procure\&quot;,\n          \&quot;url\&quot;: \&quot;\&quot;,\n          \&quot;amount\&quot;: 0,\n          \&quot;description\&quot;: \&quot;\&quot;,\n          \&quot;related_source_id\&quot;: 0,\n          \&quot;available_childs\&quot;: [],\n          \&quot;label\&quot;: \&quot;Procure\&quot;\n        }\n      ],\n      \&quot;map_type\&quot;: 1,\n      \&quot;longitude\&quot;: 0,\n      \&quot;latitude\&quot;: 0,\n      \&quot;multiple_value\&quot;: [],\n      \&quot;groups\&quot;: []\n    },\n    {\n      \&quot;id\&quot;: 0,\n      \&quot;local_id\&quot;: 1587711422293,\n      \&quot;position\&quot;: 1,\n      \&quot;block_order\&quot;: 1,\n      \&quot;input_id\&quot;: 14835,\n      \&quot;source_id\&quot;: 1415,\n      \&quot;value\&quot;: \&quot;\&quot;,\n      \&quot;name\&quot;: \&quot;multi select\&quot;,\n      \&quot;type\&quot;: \&quot;multiple_selection\&quot;,\n      \&quot;percentage\&quot;: 0,\n      \&quot;placeholder\&quot;: \&quot;select\&quot;,\n      \&quot;is_mandatory\&quot;: false,\n      \&quot;is_delete\&quot;: false,\n      \&quot;is_back_date\&quot;: false,\n      \&quot;is_backdate\&quot;: false,\n      \&quot;is_can_access_gallery\&quot;: true,\n      \&quot;camera_use_type\&quot;: 0,\n      \&quot;value_id\&quot;: 0,\n      \&quot;case_id\&quot;: 0,\n      \&quot;child_name\&quot;: \&quot;\&quot;,\n      \&quot;child_min\&quot;: 0,\n      \&quot;child_max\&quot;: 0,\n      \&quot;is_can_similar\&quot;: false,\n      \&quot;is_child_mandatory\&quot;: false,\n      \&quot;is_allow_to_edit\&quot;: true,\n      \&quot;is_editable\&quot;: true,\n      \&quot;order_id\&quot;: 0,\n      \&quot;available_list\&quot;: [\n        {\n          \&quot;_id\&quot;: \&quot;5d1da49dbb7bb06ae064b398\&quot;,\n          \&quot;id\&quot;: 9981,\n          \&quot;name\&quot;: \&quot;Times\&quot;,\n          \&quot;url\&quot;: \&quot;\&quot;,\n          \&quot;amount\&quot;: 0,\n          \&quot;description\&quot;: \&quot;\&quot;,\n          \&quot;related_source_id\&quot;: 0,\n          \&quot;available_childs\&quot;: [],\n          \&quot;label\&quot;: \&quot;Times\&quot;\n        },\n        {\n          \&quot;_id\&quot;: \&quot;5d1da49dbb7bb06ae064b399\&quot;,\n          \&quot;id\&quot;: 9982,\n          \&quot;name\&quot;: \&quot;Expense\&quot;,\n          \&quot;url\&quot;: \&quot;\&quot;,\n          \&quot;amount\&quot;: 0,\n          \&quot;description\&quot;: \&quot;\&quot;,\n          \&quot;related_source_id\&quot;: 0,\n          \&quot;available_childs\&quot;: [],\n          \&quot;label\&quot;: \&quot;Expense\&quot;\n        },\n        {\n          \&quot;_id\&quot;: \&quot;5d1da49dbb7bb06ae064b39a\&quot;,\n          \&quot;id\&quot;: 9983,\n          \&quot;name\&quot;: \&quot;Procure\&quot;,\n          \&quot;url\&quot;: \&quot;\&quot;,\n          \&quot;amount\&quot;: 0,\n          \&quot;description\&quot;: \&quot;\&quot;,\n          \&quot;related_source_id\&quot;: 0,\n          \&quot;available_childs\&quot;: [],\n          \&quot;label\&quot;: \&quot;Procure\&quot;\n        }\n      ],\n      \&quot;map_type\&quot;: 1,\n      \&quot;longitude\&quot;: 0,\n      \&quot;latitude\&quot;: 0,\n      \&quot;multiple_value\&quot;: [],\n      \&quot;groups\&quot;: []\n    },\n    {\n      \&quot;id\&quot;: 0,\n      \&quot;local_id\&quot;: 1587711422294,\n      \&quot;position\&quot;: 1,\n      \&quot;block_order\&quot;: 1,\n      \&quot;input_id\&quot;: 14836,\n      \&quot;source_id\&quot;: 0,\n      \&quot;value\&quot;: \&quot;\&quot;,\n      \&quot;name\&quot;: \&quot;user single\&quot;,\n      \&quot;type\&quot;: \&quot;user_single_selection\&quot;,\n      \&quot;percentage\&quot;: 0,\n      \&quot;placeholder\&quot;: \&quot;select\&quot;,\n      \&quot;is_mandatory\&quot;: false,\n      \&quot;is_delete\&quot;: false,\n      \&quot;is_back_date\&quot;: false,\n      \&quot;is_backdate\&quot;: false,\n      \&quot;is_can_access_gallery\&quot;: true,\n      \&quot;camera_use_type\&quot;: 0,\n      \&quot;value_id\&quot;: 0,\n      \&quot;case_id\&quot;: 0,\n      \&quot;child_name\&quot;: \&quot;\&quot;,\n      \&quot;child_min\&quot;: 0,\n      \&quot;child_max\&quot;: 0,\n      \&quot;is_can_similar\&quot;: false,\n      \&quot;is_child_mandatory\&quot;: false,\n      \&quot;is_allow_to_edit\&quot;: true,\n      \&quot;is_editable\&quot;: true,\n      \&quot;order_id\&quot;: 0,\n      \&quot;available_list\&quot;: [],\n      \&quot;map_type\&quot;: 1,\n      \&quot;longitude\&quot;: 0,\n      \&quot;latitude\&quot;: 0,\n      \&quot;multiple_value\&quot;: [],\n      \&quot;groups\&quot;: []\n    },\n    {\n      \&quot;id\&quot;: 0,\n      \&quot;local_id\&quot;: 1587711422294,\n      \&quot;position\&quot;: 1,\n      \&quot;block_order\&quot;: 1,\n      \&quot;input_id\&quot;: 14837,\n      \&quot;source_id\&quot;: 0,\n      \&quot;value\&quot;: \&quot;\&quot;,\n      \&quot;name\&quot;: \&quot;user multi\&quot;,\n      \&quot;type\&quot;: \&quot;user_multiple_selection\&quot;,\n      \&quot;percentage\&quot;: 0,\n      \&quot;placeholder\&quot;: \&quot;select\&quot;,\n      \&quot;is_mandatory\&quot;: false,\n      \&quot;is_delete\&quot;: false,\n      \&quot;is_back_date\&quot;: false,\n      \&quot;is_backdate\&quot;: false,\n      \&quot;is_can_access_gallery\&quot;: true,\n      \&quot;camera_use_type\&quot;: 0,\n      \&quot;value_id\&quot;: 0,\n      \&quot;case_id\&quot;: 0,\n      \&quot;child_name\&quot;: \&quot;\&quot;,\n      \&quot;child_min\&quot;: 0,\n      \&quot;child_max\&quot;: 0,\n      \&quot;is_can_similar\&quot;: false,\n      \&quot;is_child_mandatory\&quot;: false,\n      \&quot;is_allow_to_edit\&quot;: true,\n      \&quot;is_editable\&quot;: true,\n      \&quot;order_id\&quot;: 0,\n      \&quot;available_list\&quot;: [],\n      \&quot;map_type\&quot;: 1,\n      \&quot;longitude\&quot;: 0,\n      \&quot;latitude\&quot;: 0,\n      \&quot;multiple_value\&quot;: [],\n      \&quot;groups\&quot;: []\n    }\n  ],\n  \&quot;form_id\&quot;: 2730,\n  \&quot;local_id\&quot;: 1587711422294,\n  \&quot;smart_connect\&quot;: {\n    \&quot;category\&quot;: \&quot;{$category}\&quot;,\n    \&quot;amount\&quot;: \&quot;${amount}\&quot;\n  },\n  \&quot;name\&quot;: \&quot;${title}\&quot;,\n  \&quot;offline_created_date\&quot;: 1587711422294,\n  \&quot;timezone\&quot;: \&quot;Fri Apr 24 2020 13:57:02 GMT+0700 (Western Indonesia Time)\&quot;\n}&quot;,
  &quot;contentType&quot;: &quot;application/json&quot;,
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
   <restUrl>${url}//create</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceFunction></soapServiceFunction>
   <variables>
      <defaultValue>GlobalVariable.MicroServices</defaultValue>
      <description></description>
      <id>40880883-644b-4ba6-a3c4-4785d20f375a</id>
      <masked>false</masked>
      <name>url</name>
   </variables>
   <variables>
      <defaultValue>'Expense Connect Form'</defaultValue>
      <description></description>
      <id>3b292328-f965-4778-a70d-7e4883cc4a81</id>
      <masked>false</masked>
      <name>title</name>
   </variables>
   <variables>
      <defaultValue>true</defaultValue>
      <description></description>
      <id>7dcdba19-6e02-4e7c-9d99-f405915f0ab4</id>
      <masked>false</masked>
      <name>is_mandatory</name>
   </variables>
   <variables>
      <defaultValue>0</defaultValue>
      <description></description>
      <id>599845bb-c1b0-432d-84cf-bd42d9198ff6</id>
      <masked>false</masked>
      <name>source_id</name>
   </variables>
   <variables>
      <defaultValue>false</defaultValue>
      <description></description>
      <id>a52b4125-b177-4444-a0c6-a0de881edc56</id>
      <masked>false</masked>
      <name>is_back_date</name>
   </variables>
   <variables>
      <defaultValue>false</defaultValue>
      <description></description>
      <id>14060207-e8e7-4a3a-b575-67d49b027552</id>
      <masked>false</masked>
      <name>is_can_similar</name>
   </variables>
   <variables>
      <defaultValue>false</defaultValue>
      <description></description>
      <id>90645773-1854-4c7c-aa96-fe2a2d99aa74</id>
      <masked>false</masked>
      <name>is_delete</name>
   </variables>
   <variables>
      <defaultValue>true</defaultValue>
      <description></description>
      <id>9a0da46b-a725-4496-9a67-a1a8e4985a43</id>
      <masked>false</masked>
      <name>is_editable</name>
   </variables>
   <variables>
      <defaultValue>true</defaultValue>
      <description></description>
      <id>52b113da-7a46-4a9d-8dbd-dd00e4db57de</id>
      <masked>false</masked>
      <name>is_can_access_gallery</name>
   </variables>
   <variables>
      <defaultValue>0</defaultValue>
      <description></description>
      <id>3fc08548-9982-4f9c-a04b-64229b35fc64</id>
      <masked>false</masked>
      <name>camera_use_type</name>
   </variables>
   <variables>
      <defaultValue>1</defaultValue>
      <description></description>
      <id>c98df131-81f9-44f2-b955-70e044208fc8</id>
      <masked>false</masked>
      <name>map_type</name>
   </variables>
   <variables>
      <defaultValue>[]</defaultValue>
      <description></description>
      <id>c69e5918-09af-49e2-96e4-97914701deaa</id>
      <masked>false</masked>
      <name>available_list</name>
   </variables>
   <variables>
      <defaultValue>[]</defaultValue>
      <description></description>
      <id>94bf3e9f-9348-481e-83f8-348a9b465fad</id>
      <masked>false</masked>
      <name>multiple_list</name>
   </variables>
   <variables>
      <defaultValue>[('Id') : 98, ('Product') : 'Expense']</defaultValue>
      <description></description>
      <id>3c148b6c-56b9-41f9-8aff-b6441a727dfb</id>
      <masked>false</masked>
      <name>category</name>
   </variables>
   <variables>
      <defaultValue>null</defaultValue>
      <description></description>
      <id>d20434a6-dd2c-4491-88dd-8ca1c60c9259</id>
      <masked>false</masked>
      <name>amount</name>
   </variables>
   <variables>
      <defaultValue>'number_decimal'</defaultValue>
      <description></description>
      <id>ae8e8769-8d55-44be-aa10-76307928bc7a</id>
      <masked>false</masked>
      <name>type</name>
   </variables>
   <verificationScript>import static org.assertj.core.api.Assertions.*

import com.kms.katalon.core.testobject.RequestObject
import com.kms.katalon.core.testobject.ResponseObject
import com.kms.katalon.core.webservice.verification.WSResponseManager
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webservice.verification.WSResponseManager

RequestObject request = WSResponseManager.getInstance().getCurrentRequest()

ResponseObject response = WSResponseManager.getInstance().getCurrentResponse()

println response.getResponseBodyContent()

def title 			= request.getVariables().get('title')
def category 		= request.getVariables().get('category')
def amount 			= request.getVariables().get('amount')
def is_mandatory 	= request.getVariables().get('is_mandatory')
def source_id 		= request.getVariables().get('source_id')
def is_back_date 	= request.getVariables().get('is_back_date')
def is_can_similar 	= request.getVariables().get('is_can_similar')
def is_delete 		= request.getVariables().get('is_delete')
def is_editable 	= request.getVariables().get('is_editable')
def is_can_access 	= request.getVariables().get('is_can_access')
def camera_use_type = request.getVariables().get('camera_use_type')
def map_type 		= request.getVariables().get('map_type')
def available_list 	= request.getVariables().get('available_list')
def multiple_list 	= request.getVariables().get('multiple_list')

if (is_mandatory != null){
	if (amount != null){
		WS.verifyResponseStatusCode(response, 200)
		WS.verifyElementPropertyValue(response, 'error', false)
		WS.verifyElementPropertyValue(response, 'message', 'Successfully create')
	}
	else if (category != null){
		WS.verifyResponseStatusCode(response, 200)
		WS.verifyElementPropertyValue(response, 'error', false)
		WS.verifyElementPropertyValue(response, 'message', 'Successfully create')
	}
	else {
		WS.verifyResponseStatusCode(response, 200)
		WS.verifyElementPropertyValue(response, 'error', false)
		WS.verifyElementPropertyValue(response, 'message', 'Successfully create')
	}
}
else if (title != ''){
		if (amount != null){
			WS.verifyResponseStatusCode(response, 200)
			WS.verifyElementPropertyValue(response, 'error', false)
			WS.verifyElementPropertyValue(response, 'message', 'Successfully create')
		}
		else if (category != null){
			WS.verifyResponseStatusCode(response, 200)
			WS.verifyElementPropertyValue(response, 'error', false)
			WS.verifyElementPropertyValue(response, 'message', 'Successfully create')
		}
		else {
			WS.verifyResponseStatusCode(response, 200)
			WS.verifyElementPropertyValue(response, 'error', false)
			WS.verifyElementPropertyValue(response, 'message', 'Successfully create')
		}
}
else if ( is_mandatory == null){
	WS.verifyResponseStatusCode(response, 500)
}
else if ( title == ''){
	WS.verifyResponseStatusCode(response, 500)
}
</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
