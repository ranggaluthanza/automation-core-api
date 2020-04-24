import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import internal.GlobalVariable as GlobalVariable

WS.sendRequestAndVerify(findTestObject('MicroServices/JojoFormTransactionService/Public/create', [('url') : GlobalVariable.MicroServices
            , ('title') : 'Expense Connect Form', ('is_mandatory') : true, ('source_id') : 0, ('is_back_date') : false, ('is_can_similar') : false
            , ('is_delete') : false, ('is_editable') : true, ('is_can_access_gallery') : true, ('camera_use_type') : 0, ('map_type') : 1
            , ('available_list') : [], ('multiple_list') : [], ('category') : [('Id') : 98, ('Product') : 'Expense'], ('amount') : 40000
            , ('type') : 'number_decimal']))

