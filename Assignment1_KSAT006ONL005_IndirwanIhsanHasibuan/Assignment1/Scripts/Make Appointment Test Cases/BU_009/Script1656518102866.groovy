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
import com.kms.katalon.core.testng.keyword.TestNGBuiltinKeywords as TestNGKW
import com.kms.katalon.core.testobject.TestObject as TestObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords as WebUI
import com.kms.katalon.core.windows.keyword.WindowsBuiltinKeywords as Windows
import internal.GlobalVariable as GlobalVariable
import org.openqa.selenium.Keys as Keys

WebUI.callTestCase(findTestCase('Login Test Cases/BU_001'), [:], FailureHandling.STOP_ON_FAILURE)

WebUI.click(findTestObject('Object Repository/Page_BU_009/input_Apply for hospital readmission_hospit_63901f'))

WebUI.click(findTestObject('Object Repository/Page_BU_009/input_Visit Date (Required)_visit_date'))

WebUI.click(findTestObject('Object Repository/Page_BU_009/th_June 2022'))

WebUI.click(findTestObject('Object Repository/Page_BU_009/th_2022'))

WebUI.click(findTestObject('Object Repository/Page_BU_009/th_'))

WebUI.click(findTestObject('Object Repository/Page_BU_009/span_2018'))

WebUI.click(findTestObject('Object Repository/Page_BU_009/span_Apr'))

WebUI.click(findTestObject('Object Repository/Page_BU_009/td_11'))

WebUI.setText(findTestObject('Object Repository/Page_BU_009/textarea_Comment_comment'), 'Booking for granny')

WebUI.click(findTestObject('Object Repository/Page_BU_009/button_Book Appointment'))

WebUI.click(findTestObject('Object Repository/Page_BU_009/section_Appointment Confirmation           _5b3155'))

