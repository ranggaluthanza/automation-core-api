import static com.kms.katalon.core.checkpoint.CheckpointFactory.findCheckpoint
import static com.kms.katalon.core.testcase.TestCaseFactory.findTestCase
import static com.kms.katalon.core.testdata.TestDataFactory.findTestData
import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject
import static com.kms.katalon.core.testobject.ObjectRepository.findWindowsObject
import com.kms.katalon.core.checkpoint.Checkpoint as Checkpoint
import com.kms.katalon.core.cucumber.keyword.CucumberBuiltinKeywords as CucumberKW
import com.kms.katalon.core.mobile.keyword.MobileBuiltInKeywords as Mobile
import com.kms.katalon.core.model.FailureHandling as FailureHandling
import com.kms.katalon.core.testcase.TestCase as TestCase
import com.kms.katalon.core.testdata.TestData as TestData
import com.kms.katalon.core.testobject.TestObject as TestObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords as WebUI
import com.kms.katalon.core.windows.keyword.WindowsBuiltinKeywords as Windows
import internal.GlobalVariable as GlobalVariable

WS.sendRequestAndVerify(findTestObject('MicroServices/JojoFormTransactionService/Public/create', [('url') : GlobalVariable.MicroServices, ('title') : 'Expense Connect Form'
            , ('is_mandatory') : true, ('source_id') : 0, ('is_back_date') : false, ('is_can_similar') : false, ('is_delete') : false
            , ('is_editable') : true, ('is_can_access_gallery') : true, ('camera_use_type') : 0, ('map_type') : 1, ('available_list') : []
            , ('multiple_list') : [], ('category') : [('id') : '9512', ('name') : 'Transportation'], ('amount') : 400000]))

