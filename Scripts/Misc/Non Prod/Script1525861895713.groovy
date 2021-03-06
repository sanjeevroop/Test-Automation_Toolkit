import static com.kms.katalon.core.checkpoint.CheckpointFactory.findCheckpoint
import static com.kms.katalon.core.testcase.TestCaseFactory.findTestCase
import static com.kms.katalon.core.testdata.TestDataFactory.findTestData
import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject
import com.kms.katalon.core.checkpoint.Checkpoint as Checkpoint
import com.kms.katalon.core.checkpoint.CheckpointFactory as CheckpointFactory
import com.kms.katalon.core.mobile.keyword.MobileBuiltInKeywords as MobileBuiltInKeywords
import com.kms.katalon.core.mobile.keyword.MobileBuiltInKeywords as Mobile
import com.kms.katalon.core.model.FailureHandling as FailureHandling
import com.kms.katalon.core.testcase.TestCase as TestCase
import com.kms.katalon.core.testcase.TestCaseFactory as TestCaseFactory
import com.kms.katalon.core.testdata.TestData as TestData
import com.kms.katalon.core.testdata.TestDataFactory as TestDataFactory
import com.kms.katalon.core.testobject.ObjectRepository as ObjectRepository
import com.kms.katalon.core.testobject.TestObject as TestObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WSBuiltInKeywords
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords as WebUiBuiltInKeywords
import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords as WebUI
import internal.GlobalVariable as GlobalVariable
import org.openqa.selenium.Keys as Keys

WebUI.openBrowser('https://implementationtoolkit.azurewebsites.net/')

WebUI.navigateToUrl('https://implementationtoolkit.azurewebsites.net/')

WebUI.setText(findTestObject('Non Prod/input_loginfmt (1)'), 'Sanjeev@sdworx.com')

WebUI.click(findTestObject('Non Prod/input_idSIButton9 (1)'))

WebUI.delay(5)

WebUI.mouseOver(findTestObject('Non Prod/button_Your Setup'))

WebUI.delay(5)

WebUI.click(findTestObject('Non Prod/button_Your Setup'))

WebUI.delay(5)

WebUI.mouseOver(findTestObject('Non Prod/a_Your Payroll'))

WebUI.delay(5)

WebUI.mouseOver(findTestObject('Non Prod/a_Your Pay  Benefits'))

//WebUI.click(findTestObject('Non Prod/a_Your Pay  Benefits'))

WebUI.delay(5)


WebUI.click(findTestObject('Non Prod/a_Pension'))

